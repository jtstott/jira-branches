use std::env;
use std::str;
use git2::{BranchType, Error, ErrorCode, Repository, SubmoduleIgnore};

pub fn checkout_branch(branch_name: &str) {
    let repo = Repository::discover(env::current_dir().unwrap())
        .expect("Not a git repository");

    let head = repo.head().unwrap();
    let oid = head.target().unwrap();
    let commit = repo.find_commit(oid).unwrap();

    let is_new = !branch_exists(&repo, branch_name);

    let _branch = repo.branch(
        branch_name,
        &commit,
        false,
    );

    let branch_ref = "refs/heads/".to_owned() + branch_name;
    let refname = branch_ref.as_str();

    let obj = repo.revparse_single(refname).unwrap();

    repo.checkout_tree(
        &obj,
        None,
    ).expect("Unable to checkout tree");

    repo.set_head(refname).expect("Unable to set HEAD");

    let _ = show_branch_switch(&repo, is_new);
}

fn branch_exists(repo: &Repository, branch_name: &str) -> bool {
    let found_remote = repo.find_branch(("origin/".to_owned() + branch_name).as_str(), BranchType::Remote);

    match found_remote {
        Ok(_) => true,
        Err(_) => {
            repo.find_branch(branch_name, BranchType::Local).is_ok()
        }
    }
}

fn show_branch_switch(repo: &Repository, is_new: bool) -> Result<(), Error> {
    let head = match repo.head() {
        Ok(head) => Some(head),
        Err(ref e) if e.code() == ErrorCode::UnbornBranch || e.code() == ErrorCode::NotFound => {
            None
        }
        Err(e) => return Err(e),
    };
    let head = head.as_ref().and_then(|h| h.shorthand());

    if is_new {
        println!("Switched to a new branch '{}'", head.unwrap_or("Not currently on any branch"));
        return Ok(());
    }

    let statuses = repo.statuses(None).unwrap();
    print_short(repo, &statuses);
    println!("Switched to branch '{}'", head.unwrap_or("Not currently on any branch"));
    Ok(())
}

fn print_short(repo: &Repository, statuses: &git2::Statuses) {
    for entry in statuses
        .iter()
        .filter(|e| e.status() != git2::Status::CURRENT)
    {
        if entry.status().contains(git2::Status::IGNORED) {
            continue;
        }

        let mut istatus = match entry.status() {
            s if s.contains(git2::Status::INDEX_NEW) => 'A',
            s if s.contains(git2::Status::INDEX_MODIFIED) => 'M',
            s if s.contains(git2::Status::INDEX_DELETED) => 'D',
            s if s.contains(git2::Status::INDEX_RENAMED) => 'R',
            s if s.contains(git2::Status::INDEX_TYPECHANGE) => 'T',
            _ => ' ',
        };
        let wstatus = match entry.status() {
            s if s.contains(git2::Status::WT_NEW) => {
                if istatus == ' ' {
                    istatus = '?';
                }
                '?'
            }
            s if s.contains(git2::Status::WT_MODIFIED) => 'M',
            s if s.contains(git2::Status::WT_DELETED) => 'D',
            s if s.contains(git2::Status::WT_RENAMED) => 'R',
            s if s.contains(git2::Status::WT_TYPECHANGE) => 'T',
            _ => ' ',
        };

        // if entry.status().contains(git2::Status::IGNORED) {
        //     istatus = '!';
        //     wstatus = '!';
        // }
        if istatus == '?' && wstatus == '?' {
            continue;
        }
        let mut extra = "";

        // A commit in a tree is how submodules are stored, so let's go take a
        // look at its status.
        //
        // TODO: check for GIT_FILEMODE_COMMIT
        let status = entry.index_to_workdir().and_then(|diff| {
            let ignore = SubmoduleIgnore::Unspecified;
            diff.new_file()
                .path_bytes()
                .and_then(|s| {
                    str::from_utf8(s).ok()
                })
                .and_then(|name| repo.submodule_status(name, ignore).ok())
        });
        if let Some(status) = status {
            if status.contains(git2::SubmoduleStatus::WD_MODIFIED) {
                extra = " (new commits)";
            } else if status.contains(git2::SubmoduleStatus::WD_INDEX_MODIFIED)
                || status.contains(git2::SubmoduleStatus::WD_WD_MODIFIED)
            {
                extra = " (modified content)";
            } else if status.contains(git2::SubmoduleStatus::WD_UNTRACKED) {
                extra = " (untracked content)";
            }
        }

        let (mut a, mut b, mut c) = (None, None, None);
        if let Some(diff) = entry.head_to_index() {
            a = diff.old_file().path();
            b = diff.new_file().path();
        }
        if let Some(diff) = entry.index_to_workdir() {
            a = a.or_else(|| diff.old_file().path());
            b = b.or_else(|| diff.old_file().path());
            c = diff.new_file().path();
        }

        match (istatus, wstatus) {
            ('R', 'R') => println!(
                "RR {} {} {}{}",
                a.unwrap().display(),
                b.unwrap().display(),
                c.unwrap().display(),
                extra
            ),
            ('R', w) => println!(
                "R{} {} {}{}",
                w,
                a.unwrap().display(),
                b.unwrap().display(),
                extra
            ),
            (i, 'R') => println!(
                "{}R {} {}{}",
                i,
                a.unwrap().display(),
                c.unwrap().display(),
                extra
            ),
            (i, w) => {
                println!("{}{}      {}{}", w, i, a.unwrap().display(), extra)
            }
        }
    }

    for entry in statuses
        .iter()
        .filter(|e| e.status() == git2::Status::WT_NEW)
    {
        println!(
            "?? {}",
            entry
                .index_to_workdir()
                .unwrap()
                .old_file()
                .path()
                .unwrap()
                .display()
        );
    }
}
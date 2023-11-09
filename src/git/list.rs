use std::env;
use std::error::Error;
use git2::Repository;

pub fn list_branches(issue_id: String) -> Result<(), Box<dyn Error>> {
    let repo = Repository::discover(env::current_dir().unwrap())
        .expect("Not a git repository");

    println!("Finding for issue: {}", issue_id);

    let branches = repo.branches(None).unwrap();

    // let result = repo.find_branch("origin/test-101", BranchType::Remote).unwrap();
    // let result = repo.branch_upstream_remote("refs/heads/test-101");

    let mut filter = branches.filter(|b| b.as_ref().unwrap().0.name().unwrap().unwrap().contains(issue_id.as_str()));
    let r = filter.next().unwrap().unwrap();
    println!("RESULT: {}", r.0.name().unwrap().unwrap());

    // let mut i = 0;
    // for branch in branches {
    //     let b = branch.unwrap();
    //     i += 1;
    //     println!("Branch {}: {}", i, b.0.name().unwrap().unwrap());
    //     let d = 1;
    // }

    Ok(())
}
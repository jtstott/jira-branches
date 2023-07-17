use std::env;
use git2::{Repository};

pub fn checkout_branch(branch_name: &str) {
    let repo = Repository::open(env::current_dir().unwrap())
        .expect("Not a git repository");

    let head = repo.head().unwrap();
    let oid = head.target().unwrap();
    let commit = repo.find_commit(oid).unwrap();

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
}
use std::env;
use git2::{BranchType, Repository};

pub fn checkout_branch(branch_name: String) {
    let repo = Repository::open(env::current_dir().unwrap());

    let repo = match repo {
        Ok(repo) => repo,
        Err(err) => panic!("Problem opening the file: {:?}", err)
    };

    let head = repo.head().unwrap();
    println!("{:#?}", head.name());

    let refname = branch_name.as_str();
    let (object, reference) = repo.revparse_ext(refname).expect("Object not found");

    repo.checkout_tree(&object, None)
        .expect("Failed to checkout");

    match reference {
        // gref is an actual reference like branches or tags
        Some(gref) => repo.set_head(gref.name().unwrap()),
        // this is a commit, not a reference
        None => repo.set_head_detached(object.id()),
    }
        .expect("Failed to set HEAD");
}
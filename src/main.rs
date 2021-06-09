fn main() {
    let repo = git2::Repository::open(".").unwrap();
    let sig = repo.signature().unwrap();
    let mut index = repo.index().unwrap();
    let tree_oid = index.write_tree().unwrap();
    let tree = repo.find_tree(tree_oid).unwrap();
    let mut i: u64 = 0;
    let mut parent = repo.refname_to_id("HEAD").unwrap();
    loop {
        i += 1;
        parent = repo
            .commit(
                Some("HEAD"),
                &sig,
                &sig,
                "lol",
                &tree,
                &[&repo.find_commit(parent).unwrap()],
            )
            .unwrap();
        if i % 10000 == 0 {
            println!("{}", i);
        }
    }
}

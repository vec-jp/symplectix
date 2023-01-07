fn main() {
    let repo = match git2::Repository::open("/Users/k/git/github.com/mypmc/trunk") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };

    println!("{}", repo.path().display());
}

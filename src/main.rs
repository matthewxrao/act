mod git;
use crate::git::utils::get_git_repository_path;

fn main() {
    println!("{}", get_git_repository_path());
}

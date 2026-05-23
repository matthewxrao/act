/// @file src/git.rs
use crate::paths::OPath;
use std::env;

pub fn get_git_repository() -> OPath {
    let curr_dir = env::current_dir().expect("Failed to get current working directory");

    #[cfg(debug_assertions)]
    {
        println!("* Current working directory: {} *", curr_dir.display());
    }

    fn search_directory(dir: OPath) -> OPath {
        if !dir.ok() {
            return OPath::empty();
        }

        let dirbuf = dir.value();

        #[cfg(debug_assertions)]
        {
            println!("\n============================================");
            println!("\tSearching... {}", dirbuf.display());
            println!("============================================");
        }

        if let Ok(entries) = std::fs::read_dir(dirbuf) {
            for dentry in entries.flatten() {
                if dentry.path().file_name().and_then(|s| s.to_str()) == Some(".git") {
                    #[cfg(debug_assertions)]
                    {
                        println!("\tFound .git directory in: {}", dir);
                    }

                    return dir;
                }
            }
        }

        if let Some(parent) = dirbuf.parent() {
            return search_directory(OPath::new(parent.to_path_buf()));
        }

        OPath::empty()
    }

    search_directory(OPath::new(curr_dir))
}

/// @file src/act.rs
use crate::git::get_git_repository;
use crate::paths::OPath;
use std::fs;

fn to_act_path(path: &OPath) -> OPath {
    OPath::new(path.value().join(".act/"))
}

fn act_repository_exists(root: &OPath) -> bool {
    let act_repository_path = to_act_path(root);

    if let Ok(exists) = fs::exists(act_repository_path.value()) {
        if exists {
            return true;
        }
    }

    false
}

pub fn act_init() {
    let root = get_git_repository();
    if !root.ok() {
        panic!("failed to initialize act: no git repository")
    }

    if act_repository_exists(&root) {
        panic!("act repository exists already!")
    }

    let act_repository_path = to_act_path(&root);

    if let Ok(_) = fs::create_dir(act_repository_path.value()) {
        println!("successfully created act repository in: {}", root);
    }

    panic!("failed to initialize act repository")
}

use serde::Serialize;
use std::fs::{self, File};
use std::io::{self, Cursor};
use std::path::{Path, PathBuf};
use uuid::Uuid;

pub fn create_world(
    target_path: PathBuf,
    name: String,
    rp_uuids: Vec<String>,
    bp_uuids: Vec<String>,
) {
    let world_uuid = Uuid::new_v4().to_string();
    let world_folder = target_path.join(format!("{name}_{world_uuid}"));

    match fs::create_dir_all(&world_folder) {
        Ok(_) => (),
        Err(error) => eprintln!("[cu] Failed to write world folder: {error}"),
    }

    write_level_data(&world_folder, &name);
    write_levelname(&world_folder, &name);
    write_packs(&world_folder, rp_uuids, bp_uuids);

    set_all_times(&world_folder);
}

#[derive(Debug, Serialize)]
pub struct WorldPack {
    pack_id: String,
    version: [i32; 3],
}

mod settings;

fn write_level_data(path: &Path, name: &str) {
    let path = path.join("level.dat");
    let mut writer = Cursor::new(Vec::new());
    let mut file = File::create(&path).expect("[cu] Failed to parse resource input");
    let mut settings = settings::default_settings();

    settings.level_name = name.to_string();
    settings.write(&mut writer).unwrap();

    writer.set_position(0);

    io::copy(&mut writer, &mut file).unwrap();
}

fn write_packs(path: &Path, rp_uuids: Vec<String>, bp_uuids: Vec<String>) {
    let bp_path = path.join("world_behavior_packs.json");
    let rp_path = path.join("world_resource_packs.json");

    let rp_contents: Vec<WorldPack> = rp_uuids
        .iter()
        .map(|string| WorldPack {
            pack_id: string.clone(),
            version: [1, 0, 0],
        })
        .collect();

    let bp_contents: Vec<WorldPack> = bp_uuids
        .iter()
        .map(|string| WorldPack {
            pack_id: string.clone(),
            version: [1, 0, 0],
        })
        .collect();

    let rp_json = serde_json::to_string(&rp_contents).expect("[cu] Failed to parse resource input");
    let bp_json = serde_json::to_string(&bp_contents).expect("[cu] Failed to parse resource input");

    match fs::write(rp_path, rp_json) {
        Ok(_) => (),
        Err(error) => eprintln!("[cu] Failed to write world_resource_packs.json file: {error}"),
    }

    match fs::write(bp_path, bp_json) {
        Ok(_) => (),
        Err(error) => eprintln!("[cu] Failed to write world_behavior_packs.json file: {error}"),
    }
}

fn write_levelname(path: &Path, name: &str) {
    let dest_path = path.join("levelname.txt");

    match fs::write(dest_path, name) {
        Ok(_) => (),
        Err(error) => eprintln!("[cu] Failed to write levelname.txt file: {error}"),
    }
}

fn set_all_times(start: &Path) {
    let now = filetime::FileTime::now();
    let mut stack = vec![start.to_path_buf()];

    while let Some(dir) = stack.pop() {
        let entries = match fs::read_dir(&dir) {
            Ok(entries) => entries,
            Err(error) => {
                eprintln!("[cu] Failed to read world directory: {error}");
                continue;
            }
        };

        for entry in entries.flatten() {
            let path = entry.path();
            let is_dir = entry.file_type().is_ok_and(|t| t.is_dir());

            match filetime::set_file_mtime(&path, now) {
                Ok(_) => (),
                Err(error) => eprintln!("[cu] Failed to set file time: {error}"),
            }

            if is_dir {
                stack.push(path);
            }
        }

        match filetime::set_file_mtime(&dir, now) {
            Ok(_) => (),
            Err(error) => eprintln!("[cu] Failed to set file time: {error}"),
        }
    }
}

#[test]
fn test() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let new_path = path.join("../../temp/world");

    create_world(
        new_path,
        String::from("time test 5"),
        vec![String::from("rp uuid")],
        vec![String::from("bp uuid")],
    );
}

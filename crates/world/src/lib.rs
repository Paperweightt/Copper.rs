use include_dir::{Dir, include_dir};
use std::fs;
use std::path::PathBuf;

pub static WORLD_TEMPLATE: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/world");

pub fn create_world(
    target_path: PathBuf,
    name: String,
    rp_uuids: Vec<String>,
    bp_uuids: Vec<String>,
) {
    for file in WORLD_TEMPLATE.files() {
        let src_path = file.path();

        let mut dest_path = target_path.join(src_path);

        if let Some(parent) = dest_path.parent() {
            match fs::create_dir_all(parent) {
                Ok(_) => (),
                Err(error) => eprintln!("[cu] Failed to write parent directory: {error}"),
            }
        }

        println!("{:?}", src_path);

        let extension = src_path
            .extension()
            .and_then(|os| os.to_str())
            .unwrap_or("");

        if extension == "tmpl" {
            println!("{:?}", src_path);
            let mut contents = String::from(file.contents_utf8().unwrap());

            contents = contents.replace("{{name}}", &name);
            contents = contents.replace("{{bp_uuid}}", &bp_uuids[0]);
            contents = contents.replace("{{rp_uuid}}", &rp_uuids[0]);

            dest_path.set_extension("");

            match fs::write(dest_path, contents) {
                Ok(_) => (),
                Err(error) => eprintln!("[cu] Failed to write world file: {error}"),
            }
        } else {
            match fs::write(dest_path, file.contents()) {
                Ok(_) => (),
                Err(error) => eprintln!("[cu] Failed to write world file: {error}"),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let new_path = path.join("../../temp/world");

        create_world(
            new_path,
            String::from("test"),
            vec![String::from("uuid teehee")],
            vec![String::from("uuid teehehehe")],
        );
    }
}

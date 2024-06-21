use clap::{arg, builder::Str, ArgGroup, Command};
use flate2::read::ZlibDecoder;
use std::{
    fs, io,
    io::Read,
    path::{self, Path, PathBuf},
};

fn cli() -> Command {
    Command::new("git-rust")
        .about("git remade in Rust")
        .author("corslyn")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("init")
                .about("Create an empty Git repository")
                .arg(arg!(<DIRECTORY> "The directory in which to initialize the repo")),
        )
        .subcommand(
            Command::new("cat-file")
                .about("Provide contents or details of repository objects")
                .arg(
                    arg!(<TYPE> "The type of the object")
                        .value_parser(["blob", "commit", "tag", "tree"]),
                )
                .arg(arg!(<OBJECT> "The object to display")),
        )
        .subcommand(
            Command::new("hash-object")
                .about("Compute object ID and optionally create an object from a file")
                .arg(
                    arg!(-t --type <TYPE> "object type")
                        .value_parser(["blob", "commit", "tag", "tree"]),
                )
                .arg(arg!(-w --write "write the object into the object database"))
                .arg(arg!(<PATH> "Read object from <file>")),
        )
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("init", sub_matches)) => {
            let _ = create_repo(
                sub_matches
                    .get_one::<String>("DIRECTORY")
                    .unwrap_or(&".".to_string()),
            );
        }
        Some(("cat-file", sub_matches)) => {
            // let object_type = sub_matches.get_one::<String>("TYPE").expect("required");
            let object = sub_matches.get_one::<String>("OBJECT").expect("required");
            cat_file(object);
        }
        Some(("hash-object", sub_matches)) => {
            todo!("Implement hash-object")
        }
        _ => unreachable!(),
    }
}

fn create_repo(directory: &str) -> io::Result<()> {
    let path = Path::new(directory);
    let absolute_path = fs::canonicalize(path)?;
    if !path.exists() {
        fs::create_dir_all(path)?;
    }

    let git_path = path.join(".git");

    if git_path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            ".git directory already exists",
        ));
    }

    fs::create_dir(&git_path).unwrap();
    fs::create_dir(git_path.join("objects")).unwrap();
    fs::create_dir(git_path.join("refs")).unwrap();
    fs::write(git_path.join("HEAD"), "ref: refs/heads/main\n").unwrap();
    println!(
        "Initialized empty Git repository in {}",
        absolute_path.display()
    );
    Ok(())
}

fn cat_file(object: &str) {
    let object_path = get_object_path(object);
    let decompressed_content = decompress_object(&object_path);
    println!("{}", decompressed_content);
}

fn decompress_object(object_path: &PathBuf) -> String {
    let file = fs::File::open(object_path).expect("Object file not found");
    let mut z = ZlibDecoder::new(file);
    let mut s = String::new();
    z.read_to_string(&mut s)
        .expect("Failed to decompress object");
    s
}

fn get_object_path(object: &str) -> PathBuf {
    let (dir, file) = object.split_at(2);
    PathBuf::from(format!(".git/objects/{}/{}", dir, file))
}

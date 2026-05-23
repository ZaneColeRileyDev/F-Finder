pub mod algorithim;

use clap::Parser;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio;
#[cfg(target_os = "windows")]
use crate::algorithim::{get_roots, search_file_by_both, search_file_by_ext, search_file_by_name, OutputJson};
#[cfg(target_os = "linux")]
use crate::algorithim::{search_file_by_both, search_file_by_ext, search_file_by_name, OutputJson};

#[derive(Parser, Clone)]
struct Args {
    #[arg(long, required = false)]  // the file name you are tellin the tool to look for
    term: Option<String>,

    #[arg(long, required = false)] // the file extension the tool is looking for. I, the Developer highly recommend not running this one without the term arg otherwise it will look for every single say txt file on the entire drive or all drive
    ext:Option<String>,                    //

    // #[arg(long, required = true)] // the output dir of where the output of the found file paths will be located | it will have a default Dir
    // json_output: String,            // a folder created when the tool is run the first time

    // #[arg(long, required = false)] // outputs the JSON data of all the found file paths
    // read_json: Option<String>,
    //
    // #[arg(long, required = true)] // required as it logs the creates a log file that is named via timestamp that shows the user the verbose output of what the tool did
    // log_dir: String,                // not sure how I am going to get the log file to not be fifty gigs in size though
    #[cfg(target_os = "windows")]
    #[arg(long, required = true)] // only scans the drive with that drive letter and is required if the all drives arg is not used
    drive_letter: String,
    #[cfg(target_os = "windows")]
    #[arg(long, required = false)] // is optional and tells the program to scan all drives looking for that any file/folder with the name being searched for
    all_drives: bool

}


#[cfg(target_os = "windows")]
fn get_root(args: &Args) -> Vec<String> {
    if args.all_drives {
        get_roots()
    } else if args.drive_letter == "C" {
        vec![format!("C:/Users/{}", whoami::username().unwrap().to_string())]
    } else {
        vec![format!("{}:/", args.drive_letter)]
    }
}
#[cfg(target_os = "linux")]
fn get_root() -> Vec<String> {
    vec![format!("/home/{}", whoami::username().unwrap().to_string())]
}

fn timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
#[cfg(target_os = "windows")]
async fn save_output(output: OutputJson, default_json_dir: &str, prefix: &str) {

    let path = format!("{}/{}_{}.json", default_json_dir, prefix, timestamp());
    println!("Writing to: {}", path); // add this temporarily to verify the path

    output.output_json_file(path).unwrap();
}
#[cfg(target_os = "linux")]
async fn save_output(output: OutputJson, default_json_dir: &str) {

    let path = format!("{}/{}.json", default_json_dir, timestamp());
    println!("Writing to: {}", path); // add this temporarily to verify the path

    output.output_json_file(path).unwrap();
}

#[derive(Clone, Copy)]
enum SearchMode {
    NameOnly,
    ExtOnly,
    Both
}
#[cfg(target_os = "windows")]
#[tokio::main]
async fn main() {



    let args = Args::parse();

    let default_json_dir = format!("{}/F-Finder-Data/json", std::env::current_dir().unwrap().display());

    println!("Using default json dir: {}", default_json_dir);

    if !Path::new(format!("{}/F-Finder-Data", std::env::current_dir().unwrap().display()).as_str()).exists() {
        std::fs::create_dir("F-Finder-Data").unwrap();
    }

    if ! Path::new(&default_json_dir).exists() {
        std::fs::create_dir("F-Finder-Data/json").unwrap();
    }

    let mode = match (args.clone().term.unwrap_or("".to_string()).is_empty(), args.clone().ext.unwrap_or("".to_string()).is_empty()) {
        (false, true) => SearchMode::NameOnly,
        (true, false) => SearchMode::ExtOnly,
        (false, false) => SearchMode::Both,
        (true, true) => {
            println!("Please provide a term or ext");
            return;
        }
    };

    let roots = get_root(&args);

    let handles: Vec<_> = roots.iter().map(|root| {
        let root = root.clone();
        let term = args.term.clone();
        let ext = args.ext.clone();


        tokio::spawn(async move {
            match mode {
                SearchMode::NameOnly => search_file_by_name(root, term.unwrap()).await,
                SearchMode::ExtOnly => search_file_by_ext(root, ext).await,
                SearchMode::Both => search_file_by_both(root, term, ext).await,
            }
        })
    }).collect();

    for (handle, root) in handles.into_iter().zip(roots.iter()) {
        if let Some(output) = handle.await.unwrap() {
            save_output(output, &default_json_dir, &args.drive_letter).await;
        }
    }

}

#[cfg(target_os = "linux")]
#[tokio::main]
async fn main() {
    let args = Args::parse();

    let default_data_dir = format!("{}/F-Finder-Data", std::env::current_dir().unwrap().display());

    let default_json_dir = format!("{}/F-Finder-Data/json", std::env::current_dir().unwrap().display());

    println!("Using default json dir: {}", default_json_dir);

    if !Path::new(format!("{}/F-Finder-Data", std::env::current_dir().unwrap().display()).as_str()).exists() {
        std::fs::create_dir(default_data_dir).unwrap();
    }

    if !Path::new(&default_json_dir).exists() {
        std::fs::create_dir(&default_json_dir).unwrap();
    }

    let mode = match (args.clone().term.unwrap_or("".to_string()).is_empty(), args.clone().ext.unwrap_or("".to_string()).is_empty()) {
        (false, true) => SearchMode::NameOnly,
        (true, false) => SearchMode::ExtOnly,
        (false, false) => SearchMode::Both,
        (true, true) => {
            println!("Please provide a term or ext");
            return;
        }
    };

    let roots = get_root();

    let handles: Vec<_> = roots.iter().map(|root| {
        let root = root.clone();
        let term = args.term.clone();
        let ext = args.ext.clone();


        tokio::spawn(async move {
            match mode {
                SearchMode::NameOnly => search_file_by_name(root, term.unwrap()).await,
                SearchMode::ExtOnly => search_file_by_ext(root, ext).await,
                SearchMode::Both => search_file_by_both(root, term, ext).await,
            }
        })
    }).collect();

    for (handle, root) in handles.into_iter().zip(roots.iter()) {
        if let Some(output) = handle.await.unwrap() {
            save_output(output, &default_json_dir).await;
        }
    }
}

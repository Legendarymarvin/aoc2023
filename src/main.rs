use std::fs;
use std::process::Command;

fn main() {
    // There is not much point to doing this
    let mut days = Vec::new();

    if let Ok(files) = fs::read_dir("src/bin") {
        for file in files.filter_map(|f| f.ok()) {
            let path = file.path();
            if path.extension().unwrap() == "rs" {
                days.push(path);
            }
        }
    }

    days.sort();
    for path in days {
        let day = path.file_stem().unwrap().to_str().unwrap();
        println!("#####################\n####### {} #######\n#####################", day);
        let output = Command::new("cargo")
            .args(["run", "--bin", day])
            .output()
            .expect("Something went wrong");
        println!("Result: \n{}", String::from_utf8_lossy(&output.stdout));
    }
}
use std::path::PathBuf;

fn read_inputs(dir: &str, ext: &str) -> Vec<(Vec<u8>, String, PathBuf)> {
    use std::io::Read;
    use std::fs::{self};
    use std::path::Path;
    use std::fs::File;

    let dir = Path::new(dir);
    assert!(dir.is_dir());
    let mut results = vec![];
    for entry in fs::read_dir(dir).expect("must read the directory") {
        let entry = entry.expect("directory should contain files");
        let path = entry.path();
        if path.is_dir() {
            continue
        } else {
            let extension = path.extension();
            if extension.is_none() {
                if ext != "" {
                    continue
                }
            } else {
                let extension = extension.unwrap();
                if extension != ext {
                    continue
                }
            }
        }
        let mut buffer = Vec::new();
        let file_name = path.file_name().unwrap().to_str().unwrap().to_owned();
        println!("Executing from {}", file_name);
        let mut f = File::open(&path).expect("must open file");
        f.read_to_end(&mut buffer).expect("must read bytes from file");
        results.push((buffer, file_name, path));
    }
    
    results
}

fn read_inputs_from_dirs(dirs: Vec<&str>, exts: Vec<&str>) -> Vec<(Vec<u8>, String, PathBuf)> {
    use std::io::Read;
    use std::fs::{self};
    use std::path::Path;
    use std::fs::File;

    let mut results = vec![];

    for dir in dirs.iter() {
        let dir = Path::new(dir);
        assert!(dir.is_dir());

        let files = fs::read_dir(dir);
        if files.is_err() {
            continue
        }
        
        for entry in files.expect("must read the directory") {
            let entry = entry.expect("directory should contain files");
            let path = entry.path();
            if path.is_dir() {
                continue
            } else {
                let extension = path.extension();
                if extension.is_none() {
                    if !exts.is_empty() {
                        continue
                    }
                } else {
                    let extension = extension.unwrap();
                    if !exts.contains(&extension.to_str().unwrap()) {
                        continue
                    }
                }
            }
            let mut buffer = Vec::new();
            // let full_path = path.to_str().unwrap().to_owned();
            // let full_path = dir.join(path.file_name().unwrap());
            let file_name = path.file_name().unwrap().to_str().unwrap().to_owned();
            println!("Executing from {}", file_name);
            let mut f = File::open(&path).expect("must open file");
            f.read_to_end(&mut buffer).expect("must read bytes from file");
            results.push((buffer, file_name, path));
        }
    }
    
    results
}

#[test]
fn cross_check_on_honggfuzz() {
    use super::run;
    use std::path::Path;

    let path = "../honggfuzz/hfuzz_workspace/fuzz_target_compare/";
    let ext = "fuzz";
    let inputs = read_inputs(path, ext);
    println!("Running on {} crash inputs", inputs.len());
    for (i, input) in inputs.iter().enumerate() {
        if i % 1000 == 0 {
            println!("Made {} iterations", i);
        }
        let (data, _file_name, full_path) = input;
        run(&data[..]);
        // std::fs::remove_file(full_path).expect("should delete fixed bug trace");
    }
}

#[test]
fn cross_check_with_op() {
    use super::run_with_op;

    let paths = vec!["fuzz_vectors/", "fuzz_vectors/fuzz_target_compare_ops/"];
    let exts = vec!["fuzz"];
    let inputs = read_inputs_from_dirs(paths, exts);
    println!("Running on {} crash inputs", inputs.len());
    for (i, input) in inputs.iter().enumerate() {
        if i % 100 == 0 {
            println!("Made {} iterations", i);
        }
        let (data, _file_name, full_path) = input;
        run_with_op(&data[..]);
        // std::fs::remove_file(full_path).expect("should delete fixed bug trace");
    }
}

#[test]
fn cross_check_on_libfuzzer() {
    use super::run;

    let path = "../fuzz/artifacts/fuzz_target_compare/";
    let ext = "";
    let inputs = read_inputs(path, ext);
    println!("Running on {} crash inputs", inputs.len());
    for (i, input) in inputs.iter().enumerate() {
        if i % 1000 == 0 {
            println!("Made {} iterations", i);
        }
        let (data, _, full_path) = input;
        run(&data[..]);
        // std::fs::remove_file(full_path).expect("should delete fixed bug trace");
    }
}

#[test]
fn cross_check_on_afl() {
    use super::run;

    let paths = vec!["../afl/out_compare/fuzzer01/crashes/", "../afl/out_compare/fuzzer02/crashes/"];
    let exts = vec![""];
    let inputs = read_inputs_from_dirs(paths, exts);
    println!("Running on {} crash inputs", inputs.len());
    for (i, input) in inputs.iter().enumerate() {
        if i % 1000 == 0 {
            println!("Made {} iterations", i);
        }
        let (data, _file_name, full_path) = input;
        run(&data[..]);
        // std::fs::remove_file(full_path).expect("should delete fixed bug trace");
    }
}
use native_dialog::{FileDialog};
use notify::{watcher, Watcher, DebouncedEvent};
use std::{io::stdin, iter::Map, collections::HashMap, fs, sync::mpsc::channel, time::Duration};


fn main() {
    let mut line: String = String::new();

    println!("Select mode:\n0: TRACK_VALUE\n1: DIFF\n2: TRACK_LOC");
    let mode = read_num();

    match mode {
        0 => {
            mode_track_value();
        }
        1 => {
            mode_diff();
        }
        2 => {
            mode_track_loc();
        }
        _ => {
            println!("Mode {} is not valid", mode);
        }
    }

    println!("Press enter to exit...");
    stdin().read_line(&mut line).unwrap();
}

fn mode_diff() {
    println!("Select two files to compare...");
    
    let path_one = FileDialog::new()
        .show_open_single_file()
        .unwrap();
        
    let path_two = FileDialog::new()
        .show_open_single_file()
        .unwrap();
        
    if let Some(path_one) = path_one {
        if let Some(path_two) = path_two {
            let file_one = std::fs::read(path_one).expect("Unable to open the first file");
            let file_two = std::fs::read(path_two).expect("Unable to open the second file");
            
            let length = std::cmp::min(file_one.len(), file_two.len());
            
            println!("Scanning for differences");
            for i in 0 .. length {
                if file_one[i] != file_two[i] {
                    // , fileA[i], fileB[i]
                    //  - File A has value {:02X}, File B has value {:02X}
                    println!("{:X}", i);
                }
            }
            
            println!("Reached end of smallest file at {} bytes", length);
        }
    }
}

fn mode_track_value() {
    let mut target_value: u32;
    let mut valid_locations: Vec<usize> = vec![];

    println!("What type of number are you looking for?\nDECIMAL\n0: u8 \n1: u16\n2: u32");
    let search_fn = match read_num() {
        0 => {
            println!("Chosen 8 bit unsigned decimal");
            search_u8
        },
        _ => {
            println!("Invalid option. Defaulting to u8");
            search_u8
        },
    };

    loop {
        println!("Enter target value: ");
        target_value = read_num();
    
        println!("Open current verison of the file...");
        let path = FileDialog::new()
            .show_open_single_file()
            .unwrap();
    
        if let Some(path) = path {
            let bytes = std::fs::read(&path).expect(&format!("Could not read selected file path: `{}`", &path.display().to_string()));
    
            search_fn(bytes, target_value, &mut valid_locations);

            if valid_locations.len() <= 1 {
                if valid_locations.len() == 0 {
                    println!("Found no locations with value {}", target_value);
                } else {
                    println!("Found one location with value {}: {}",  target_value, &valid_locations[0]);
                }
                break;
            } else {
                println!("Found locations with value {}: ", target_value);
                for loc in &valid_locations {
                    println!("{:X}", loc);
                }
            }            
        }
    }

    println!("Find another value? (y/n)");
    
}

fn read_line() -> String {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    return line;
}

fn read_num() -> u32 {
    let line = read_line();
    line.trim().parse::<u32>().expect(&format!("Could not parse number input `{}`", line.trim()))
}

fn search_u8(bytes: Vec<u8>, target_value: u32, valid_locations: &mut Vec<usize>) {
    if valid_locations.is_empty() {
        for (loc, byte) in bytes.iter().enumerate() {
            if *byte as u32 == target_value {
                valid_locations.push(loc);
            }
        }
    } else {
        valid_locations.retain(|loc| bytes[*loc] as u32 == target_value);
    }
}

fn mode_track_loc() {
    println!("Select file to track...");
    let original_file = FileDialog::new()
        .show_open_single_file()
        .unwrap();

    if let Some(original_file) = original_file {
        let mut changes: HashMap<usize, u8> = HashMap::new();
        let mut bytes = fs::read(&original_file).expect("Could not read file");

        let (tx, rx) = channel();
        let mut watcher = watcher(tx, Duration::from_secs(1)).unwrap();

        watcher.watch(&original_file, notify::RecursiveMode::NonRecursive).unwrap();

        loop {
            match rx.recv() {
                Ok(event) => {
                    match event {
                        DebouncedEvent::Write(_) => {
                            println!("== File change detected! ==");
                            let new_bytes = fs::read(&original_file).expect("Error occurred reading updated file...");
                            
                            if (&changes).is_empty() {
                                for i in 0 .. bytes.len() {
                                    if bytes[i] != new_bytes[i] {
                                        (&mut changes).insert(i, new_bytes[i]);
                                        println!("{:X} now has value {:02X}", i, new_bytes[i]);
                                    }
                                }
                                // we don't use bytes after this point
                                bytes.clear();
                            } else {
                                (&mut changes).retain(|i, old_value| {
                                    *old_value != new_bytes[*i]
                                });

                                println!("Continued changes from last version of the file: ");
                                for (i, change) in &changes {
                                    println!("{:X} now has value {:02X}", i, change);
                                }

                                if (&changes).len() <= 1 {
                                    break;
                                }
                            }
                        }
                        _ => {
                            println!("{:?}", event);
                        }
                    }
                }
                Err(event) => eprintln!("Watch error: {:?}", event)
            }
        }
    }
}

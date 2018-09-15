extern crate csv;

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Write;

fn main() {
    let path = env::args()
        .nth(1)
        .expect("No argument provided for playlist path!");
    let mut playlists_src = csv::Reader::from_path(format!("{}{}", path, "playlists.csv"))
        .expect("Playlist file processing error!");

    let mut autoplaylist: Vec<String> = Vec::new();
    let mut playlists: HashMap<String, Vec<String>> = HashMap::new();
    let header = playlists_src.headers().expect("No header!").clone();

    for row in playlists_src.records() {
        if let Ok(row) = row {
            let mut link = String::new();
            let mut dislikes = String::new();
            let mut playlist_names: Vec<String> = Vec::new();
            let mut put_on_autoplaylist = false;

            for i in 0..row.len() {
                if row[i].is_empty() {
                    continue;
                }
                match header[i].as_ref() {
                    "Youtube link" => link = row[i].to_string(),
                    "Who dislikes this song?" => dislikes = row[i].to_string(),
                    "Game" => {
                        for name in row[i].to_string().split(',') {
                            playlist_names.push(format!("{} - {}", header[i].to_string(), name))
                        }
                        if row.get(i).unwrap() == "RuneScape" {
                            put_on_autoplaylist = true;
                        }
                    }
                    "Song Name" => (),
                    _ => {
                        for name in row[i].to_string().split(',') {
                            playlist_names.push(format!("{} - {}", header[i].to_string(), name))
                        }
                    }
                }
            }

            if put_on_autoplaylist && dislikes.is_empty() {
                autoplaylist.push(link.clone());
            }

            for name in playlist_names {
                playlists
                    .entry(name)
                    .or_insert(Vec::new())
                    .push(link.clone());
            }
        }
    }

    let mut file = File::create(format!("{}{}", path, "autoplaylist.txt")).unwrap();
    for i in autoplaylist {
        file.write_all((i + "\n").as_bytes());
    }

    for (key, val) in playlists.iter() {
        println!("{}{}.txt", path, key);
        let mut file = File::create(format!("{}{}.txt", path, key)).unwrap();
        for i in val.iter() {
            file.write_all(format!("{}\n", i).as_bytes());
        }
    }
}

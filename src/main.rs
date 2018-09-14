extern crate csv;

use std::collections::HashMap;
use std::env;

fn main() {
    let path = env::args()
        .nth(1)
        .expect("No argument provided for playlist path!");
    let mut playlists_src =
        csv::Reader::from_path(path + "playlists.csv").expect("Playlist file processing error!");

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
                match header[i].as_ref() {
                    "Youtube link" => link = row[i].to_string(),
                    "Who dislikes this song?" => dislikes = row[i].to_string(),
                    "Game/Artist" => {
                        playlist_names.push(format!(
                            "{} - {}",
                            header[i].to_string(),
                            row[i].to_string()
                        ));
                        if row.get(i).unwrap() == "RuneScape" {
                            put_on_autoplaylist = true;
                        }
                    }
                    "Song Name" => (),
                    _ => playlist_names.push(format!(
                        "{} - {}",
                        header[i].to_string(),
                        row[i].to_string()
                    )),
                }
            }

            if put_on_autoplaylist && dislikes.is_empty() {
                autoplaylist.push(link.clone());
            }

            for name in playlist_names {
                for i in name.split(',') {
                    playlists
                        .entry(i.to_string())
                        .or_insert(Vec::new())
                        .push(link.clone());
                }
            }
        }
    }

    for (key, val) in playlists.iter() {
        println!("key: {} val: {:?}", key, val);
    }
}

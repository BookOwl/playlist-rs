use url;
use std::path;
use std::io;
use std::io::{Read, BufRead, BufReader};
use url::Url;

#[derive(PartialEq, Eq, Debug)]
pub enum Entry {
    File(path::PathBuf),
    Url(url::Url),
}

#[derive(PartialEq, Eq, Debug)]
pub struct Playlist {
    items: Vec<Entry>,
}

impl Playlist {
    pub fn load_from_m3u(r: impl io::Read) -> Result<Self, io::Error> {
        let r = BufReader::new(r);
        let mut items = Vec::new();
        for line in r.lines() {
            let entry;
            let line = line?;
            if line.trim() == "" {
                continue
            }
            if let Ok(u) = Url::parse(line.trim()) {
                entry = Entry::Url(u);
            } else {
                entry = Entry::File(path::PathBuf::from(line.trim()));
            }
            items.push(entry);
        }
        Ok(Playlist{items})
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    #[test]
    fn test_m3u_load() {
        let raw_playlist = "
            SomeArtist/SomeAlbum/SomeSong.mp3
            https://www.youtube.com/watch?v=WSUFzC6_fp8
            https://www.youtube.com/watch?v=ifCWN5pJGIE
            AnotherAlbum/Song.flac
            ~/Music/Foo/Bar/Baz/Biddle/spam.mp3
            ";
        let expected = Playlist {
            items: vec![
                Entry::File(path::PathBuf::from("SomeArtist/SomeAlbum/SomeSong.mp3")),
                Entry::Url(Url::parse("https://www.youtube.com/watch?v=WSUFzC6_fp8").unwrap()),
                Entry::Url(Url::parse("https://www.youtube.com/watch?v=ifCWN5pJGIE").unwrap()),
                Entry::File(path::PathBuf::from("AnotherAlbum/Song.flac")),
                Entry::File(path::PathBuf::from("~/Music/Foo/Bar/Baz/Biddle/spam.mp3")),
            ]
        };
        let got = Playlist::load_from_m3u(io::Cursor::new(raw_playlist)).unwrap();
        assert_eq!(got, expected);
    }
}

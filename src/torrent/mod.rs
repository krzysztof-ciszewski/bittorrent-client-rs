use crate::bencode::Dict;

pub struct Tracker(String);

pub struct Torrent {
    pub trackers: Vec<Tracker>,
    pub name: String,
    pub private: bool,
    pub length: u64,
    pub piece_length: u64,
    pub piece_hashes: Vec<String>,
    pub comment: String,
    pub created_by: String,
    pub creation_date: u64,
    pub encoding: String,
}

impl Torrent {
    pub fn create(dict: &Dict) -> Torrent {
        let map = &dict.values;
        let info = &map.get("info").unwrap().get_dict().unwrap().values;

        let mut trackers = vec![
            Tracker(map.get("announce").unwrap().get_string().unwrap()),
        ];

        if map.get("announce-list").is_some() {
            for list in map.get("announce-list").unwrap().get_list().unwrap().values {
                trackers.push(Tracker(list.get_list().unwrap().values[0].get_string().unwrap()));
            }
        }

        let piece_length = info.get("piece length").unwrap().get_number().unwrap().clone();
        let hashes = info.get("pieces").unwrap().get_string().unwrap().as_bytes().to_vec();
        let mut piece_hashes = vec![];
        for hash in hashes.windows(piece_length.into()) {
            unsafe {
                piece_hashes.push(String::from_utf8_unchecked(hash.to_vec()));
            }
        }


        Torrent {
            trackers,
            created_by: map.get("created by").unwrap().get_string().unwrap(),
            encoding: map.get("encoding").unwrap().get_string().unwrap(),
            creation_date: map.get("creation date").unwrap().get_number().unwrap().clone(),
            comment: map.get("comment").unwrap().get_string().unwrap(),
            name: info.get("name").unwrap().get_string().unwrap(),
            private: info.get("private").unwrap().get_number().unwrap() == &1,
            length: info.get("length").unwrap().get_number().unwrap().clone(),
            piece_length,
            piece_hashes
        }
    }
}


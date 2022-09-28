extern crate musicbrainz_rs;
use musicbrainz_rs::entity::artist::*;
use musicbrainz_rs::entity::release::Release;
use musicbrainz_rs::entity::release_group::ReleaseGroup;
use musicbrainz_rs::prelude::*;

use std::collections::HashMap;
use std::hash::Hash;

use lastfm_rs::Lastfm;


fn main() {


    //getting the artist mbid
    let artist_mbid = get_artist_mbid();
    //searching for release groups from the artist
    let artist = Artist::fetch()
        .id(&artist_mbid)
        .with_release_groups()
        .execute()
        .unwrap();
    
    //getting the mbids from the releases
    let mut release_mbids: HashMap<String, String> = HashMap::new();
    let mut title_vec = vec![];
    let mut id_vec = vec![];
    for release in artist.release_groups.unwrap() {
        let release_group = &ReleaseGroup::fetch()
            .id(&release.id)
            .with_releases()
            .execute()
            .unwrap()
            .releases
            .unwrap()
            [0];
        title_vec.push(release_group.title.clone());
        id_vec.push(release_group.id.clone());
    }
    let mut i = 0;
    for title in title_vec {
        let id = &id_vec[i];
        i += 1;

        release_mbids.insert(title,id.to_string());
    }
    
    //get title of songs and posistion of songs
    for mbid in release_mbids {
        let recordings = Release::fetch()
            .id(&mbid.1)
            .with_recordings()
            .execute()
            .unwrap();
        for media in recordings.media.unwrap() {
            for track in media.tracks.unwrap() {
                println!("{:?}, {:?}",track.recording.title, media.position, );
            }
        }
    }

    
}

#[tokio::main]
async fn get_artist_mbid() -> String {
    let client = Lastfm::new("905681146fae0fd61f4fb6eac369457c");

    let lastfm_artist = client.artist.get_info("Lorde").await.unwrap();
    let artist_mbid = lastfm_artist.artist.mbid.unwrap();
    return artist_mbid
}

fn get_album_data(mbid: String) {
    return
}
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

extern crate musicbrainz_rs;
use musicbrainz_rs::{entity::{artist::*, release::Release, release_group::ReleaseGroup}, prelude::*};

use serde::{Serialize, Deserialize};

////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Serialize, Deserialize, Debug)]
pub struct SongData {
    mbid: String, //musicbrainz id
    number: i32   //tracklist number
}

////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Serialize, Deserialize, Debug)]
pub struct ReleaseData {
    mbid: String, //musicbrainz id
    type_of_release: String,
    tracks: HashMap<String, SongData>
}

////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Serialize, Deserialize, Debug)]
pub struct ArtistData {
    mbid: String, //musicbrainz id
    artist_type: String,
    releases:  HashMap<String, ReleaseData>
}

////////////////////////////////////////////////////////////////////////////////////////////////

fn main() {

//////////////////getting the artist mbid and name//////////////////////////////////////////////
    
    let artist_name = "Rare Americans".to_string();
    let artist_mbid = get_artist_mbid(&artist_name);

//////////////////getting info about the artist from musicbrainz////////////////////////////////

    let artist_info = Artist::fetch()
        .id(&artist_mbid.as_ref().unwrap().to_string())
        .with_release_groups()
        .execute()
        .unwrap();
        
//////////////////setting the artist type///////////////////////////////////////////////////////
 
    let artist_type = artist_info.artist_type.as_ref().unwrap().to_string();

//////////////////getting mbids for every release group/////////////////////////////////////////
 
    let mut release_group_mbids = vec![];
    for release_group in artist_info.release_groups.unwrap() {
        release_group_mbids.push(release_group.id);
    }
 
//////////////////getting mbids for every release///////////////////////////////////////////////
 
    let mut release_mbids = vec![];
    for mbid in &release_group_mbids {
        let release_group_info = &mut ReleaseGroup::fetch()
            .id(&mbid)
            .with_releases()
            .execute()
            .unwrap();
        
        release_mbids.push((release_group_info.releases.as_ref().unwrap()[0].id.clone(), 
                            release_group_info.primary_type.as_ref().unwrap().to_string()))

    }

//////////////////getting data from the releases to make ReleaseData////////////////////////////
    
    //let mut artist_releases = vec![];
    let mut releases_hashmap = HashMap::new();
    for mbid in release_mbids { //for release in releases
        let mut song_hashmap = HashMap::new();
        let release = &mut Release::fetch()
            .id(&mbid.0)
            .with_recordings()
            .execute()
            .unwrap();

        for song in release.media.as_ref().unwrap()[0].tracks.as_ref().unwrap() { //for every track make a SongData struct
            let song_data = SongData{ mbid:song.id.clone(), number:song.position as i32};
            song_hashmap.insert(song.title.clone(), song_data);
        
        }
    
        let release_data = ReleaseData {
            mbid: mbid.0,
            type_of_release: mbid.1,
            tracks: song_hashmap,
        };

        let title_of_release = format!("{} - {}", release.title.clone(), release_data.type_of_release); 

        releases_hashmap.insert(title_of_release, release_data);
    }

    let artist_data = ArtistData {
        mbid: artist_mbid.as_ref().unwrap().to_string(),
        artist_type: artist_type,
        releases: releases_hashmap,
    };

//////////////////Converting the ArtistData to a Json and uploading it to a file////////////////

    let mut dict = HashMap::new();

    dict.insert(artist_name, artist_data);

    let buf = Vec::new();
    let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
    let mut ser = serde_json::Serializer::with_formatter(buf, formatter);
    dict.serialize(&mut ser).unwrap();

    let mut file = File::create("./temp.json").unwrap();

    write!(file, "{}", String::from_utf8(ser.into_inner()).unwrap()).unwrap();
}

//////////////////function to get the artists mbid//////////////////////////////////////////////

fn get_artist_mbid(artist_name: &String) -> Option<String> {
    
    let query = ArtistSearchQuery::query_builder()
        .artist(artist_name)
        .build();

    let query_result = Artist::search(query).execute().unwrap();

    //println!("{:?}",query_result);

    let artist_mbid = &query_result.entities[0].id;

    return Some(artist_mbid.to_string())
}

////////////////////////////////////////////////////////////////////////////////////////////////
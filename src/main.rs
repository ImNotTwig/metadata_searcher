extern crate musicbrainz_rs;
use std::collections::HashMap;

use musicbrainz_rs::{entity::{artist::*, release::Release, release_group::ReleaseGroup}, prelude::*};

use lastfm_rs::Lastfm;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SongData {
    name: String, //name of the song
    mbid: String, //musicbrainz id
    number: i32   //tracklist number
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReleaseData {
    name: String, //name of the release
    mbid: String, //musicbrainz id
    tracks: Vec<SongData>   //[
                            //{name, mbid, number}, 
                            //{name, mbid, number}
                            //]
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReleaseDataWithoutName {
    mbid: String, //musicbrainz id
    tracks: Vec<SongData>   //[
                            //{name, mbid, number}, 
                            //{name, mbid, number}
                            //]
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ArtistData {
    name: String, //name of the artist
    mbid: String, //musicbrainz id
    artist_type: String,
    releases: Vec<ReleaseData> //[
                            //{name, mbid,[                     //list of ReleaseData 
                                        //{name, mbid, number}, 
                                        //{name, mbid, number}
                                        //]
                                        //},
                            //{name, mbid,[                     //list of ReleaseData
                                        //{name, mbid, number}, 
                                        //{name, mbid, number}
                                        //]
                                        //}}
                            //] 
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ArtistDataWithoutName {
    mbid: String,
    artist_type: String,
    releases: Vec<HashMap<String, ReleaseDataWithoutName>>
}

fn main() {

//////////////////getting the artist mbid and name//////////////////////////////////////////////
    
    let artist_name = "Lorde".to_string();
    let artist_mbid = get_artist_mbid();

//////////////////getting info about the artist from musicbrainz////////////////////////////////

    let artist_info = Artist::fetch()
        .id(&artist_mbid)
        .with_release_groups()
        .execute()
        .unwrap();
    //println!("{:?}", artist_info);
        
//////////////////setting the artist type///////////////////////////////////////////////////////
 
    let artist_type = artist_info.artist_type.unwrap().to_string();

//////////////////getting mbids for every release group/////////////////////////////////////////
 
    let mut release_group_mbids = vec![];
    for release in artist_info.release_groups.unwrap() {
        release_group_mbids.push(release.id);
    }
 
//////////////////getting mbids for every release///////////////////////////////////////////////
 
    let mut release_mbids = vec![];
    for mbid in &release_group_mbids {
        let release_group_info = &mut ReleaseGroup::fetch()
            .id(&mbid)
            .with_releases()
            .execute()
            .unwrap();
        
        release_mbids.push(release_group_info.releases.as_ref().unwrap()[0].id.clone())
        //println!("{:?}, {:?}, {:?}", release_info.title, track.recording.title, track.recording.id);
    }

//////////////////getting data from the releases to make ReleaseData////////////////////////////
    
    let mut artist_releases = vec![];
    for mbid in release_mbids { //for release in releases
        let mut tracks = vec![];
        let release = &mut Release::fetch()
            .id(&mbid)
            .with_recordings()
            .execute()
            .unwrap();



        for song in release.media.as_ref().unwrap()[0].tracks.as_ref().unwrap() { //for every track make a SongData struct
            tracks.push(SongData{ name:song.title.clone(), mbid:song.id.clone(), number:song.position as i32})
        }
        

        let release_data_without_name = ReleaseDataWithoutName {
            mbid: mbid,
            tracks: tracks,
        };

        let mut dict = HashMap::new();
        dict.insert(release.title.clone(),release_data_without_name);
        artist_releases.push(dict)

    }
    
    let artist_data_without_name = ArtistDataWithoutName {
        mbid: artist_mbid,
        artist_type: artist_type,
        releases: artist_releases,
    };

    //"Lorde": ["mbid":mbid, "type":type, "releases":["name":release, "name":release]
    let mut dict = HashMap::new();

    dict.insert(artist_name, artist_data_without_name);

    let buf = Vec::new();
    let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
    let mut ser = serde_json::Serializer::with_formatter(buf, formatter);
    dict.serialize(&mut ser).unwrap();
    println!("{}", String::from_utf8(ser.into_inner()).unwrap());


    


////////////////////////////////////////////////////////////////////////////////////////////////
}





#[tokio::main]
async fn get_artist_mbid() -> String {
    let client = Lastfm::new("905681146fae0fd61f4fb6eac369457c");

    let lastfm_artist = client.artist.get_info("Lorde").await.unwrap();
    let artist_mbid = lastfm_artist.artist.mbid.unwrap();
    return artist_mbid
}
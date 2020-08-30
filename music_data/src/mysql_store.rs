
use mysql::*;
use mysql::prelude::*;

use crate::sqlite_store::Song;


pub struct MysqlStore {
    pub dbname : String, 
    pub host : String, 
    pub user : String, 
    pub passwd : String, 
    url : String,  
}

impl MysqlStore {

    pub fn new(dbname : String, host : String, user : String, passwd : String) -> MysqlStore {
        let url = format!("mysql://{}:{}@{}:3306/{}", user, passwd, host, dbname);
        println!("url : {}", url);

        MysqlStore{
            dbname : dbname.to_string(), 
            host : host.to_string(), 
            user : user.to_string(), 
            passwd : passwd.to_string(), 
            url : url.to_string(),
        }
    }

    pub fn open(&self) -> Pool {
        let pool = Pool::new(self.url.to_string()).expect("connect failed");
        return pool;
    }
    pub fn insert_song(&self, conn : &mut PooledConn, rec : Song) {

        let mut stmt = conn.prep(
            "insert into songs (
                track_id, title, song_id, 
                srelease, artist_id, artist_mbid, 
                artist_name, duration, artist_familiarity, 
                artist_hotttnesss, year, track_7digitalid, 
                shs_perf, shs_work
            ) values (
                :track_id, :title, :song_id, 
                :srelease, :artist_id, :artist_mbid, 
                :artist_name, :duration, :artist_familiarity, 
                :artist_hotttnesss, :year, :track_7digitalid, 
                :shs_perf, :shs_work
            )").expect("stmt error ");

            conn.exec_drop(stmt, params!{
                "track_id" => rec.track_id.to_string(), 
                "title" => rec.title.to_string(), 
                "song_id" => rec.song_id.to_string(), 
                "srelease" => rec.release.to_string(), 
                "artist_id" => rec.artist_id.to_string(), 
                "artist_mbid" => rec.artist_mbid.to_string(), 
                "artist_name" => rec.artist_name.to_string(), 
                "duration" => rec.duration, 
                "artist_familiarity" => rec.track_7digitalid, 
                "artist_hotttnesss" => rec.artist_hotttnesss, 
                "year" => rec.year, 
                "track_7digitalid" => rec.artist_hotttnesss, 
                "shs_perf" => rec.shs_perf, 
                "shs_work" => rec.shs_work,
            }).expect("execute error!");
    } 


    pub fn insert_songs(&self, conn : &mut PooledConn, res :&mut Vec<Song>) {
        conn.exec_batch(

            r"insert into song (
                track_id, title, song_id, 
                srelease, artist_id, artist_mbid, 
                artist_name, duration, artist_familiarity, 
                artist_hotttnesss, year, track_7digitalid, 
                shs_perf, shs_work
            ) values (
                :track_id, :title, :song_id, 
                :srelease, :artist_id, :artist_mbid, 
                :artist_name, :duration, :artist_familiarity, 
                :artist_hotttnesss, :year, :track_7digitalid, 
                :shs_perf, :shs_work
            )", 
            res.iter().map(|p| params! {
                "track_id" => p.track_id.to_string(), 
                "title" => p.title.to_string(), 
                "song_id" => p.song_id.to_string(), 
                "srelease" => p.release.to_string(), 
                "artist_id" => p.artist_id.to_string(), 
                "artist_mbid" => p.artist_mbid.to_string(), 
                "artist_name" => p.artist_name.to_string(), 
                "duration" => p.duration, 
                "artist_familiarity" => p.track_7digitalid, 
                "artist_hotttnesss" => p.artist_hotttnesss, 
                "year" => p.year, 
                "track_7digitalid" => p.artist_hotttnesss, 
                "shs_perf" => p.shs_perf, 
                "shs_work" => p.shs_work
            })
        ).expect("run sql error");

    } 


    pub fn test(&self) {
        
        let pool = Pool::new(self.url.to_string()).expect("connect failed");
        let mut conn = pool.get_conn().expect("connect get error");
        conn.query_drop(
            r"CREATE TABLE songs (
                track_id varchar(255) primary key, 
                title varchar(1000), 
                song_id varchar(255), 
                srelease varchar(255), 
                artist_id varchar(255), 
                artist_mbid varchar(255), 
                artist_name varchar(255), 
                duration float, 
                artist_familiarity float, 
                artist_hotttnesss float, 
                year int, 
                track_7digitalid int, 
                shs_perf int, 
                shs_work int  
            )"
        ).expect("query drop run error");

    }

}



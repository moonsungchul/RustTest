
use rusqlite::{params, Connection, Result, MappedRows};
use crate::mysql_store::MysqlStore;


pub struct SqliteStore{
    pub dbfile: String,        
}

#[derive(Debug)]
pub struct Song {
    pub track_id : String, 
    pub title : String, 
    pub song_id : String, 
    pub release : String, 
    pub artist_id: String, 
    pub artist_mbid : String, 
    pub artist_name : String, 
    pub duration : f64, 
    pub artist_familiarity : f64, 
    pub artist_hotttnesss : f64, 
    pub year : i64, 
    pub track_7digitalid : i64, 
    pub shs_perf : i64, 
    pub shs_work : i64,  
}

impl Song {
    pub fn new() -> Song {
        Song{
            track_id : "".to_string(), 
            title : "".to_string(), 
            song_id : "".to_string(), 
            release : "".to_string(), 
            artist_id: "".to_string(), 
            artist_mbid : "".to_string(), 
            artist_name : "".to_string(), 
            duration : 0.0, 
            artist_familiarity : 0.0, 
            artist_hotttnesss : 0.0, 
            year : 0, 
            track_7digitalid : 0, 
            shs_perf : 0, 
            shs_work : 0,  
        }
    }
}

impl SqliteStore {
   pub fn new(dbfile : String) -> SqliteStore {
       SqliteStore{dbfile}
   } 

   pub fn open(&self) -> Connection {
       let conn = Connection::open(self.dbfile.clone()).expect("connection open failed");
       return conn;
   }

   pub fn exportMySQL(&self, conn : Connection, mstore: MysqlStore ) -> Result<()> {

       let mut stmt = conn.prepare("select * from songs;")?;
       let song_iter = stmt.query_map(params![], |row| {
            Ok(Song {
            track_id : row.get(0)?, 
            title : row.get(1)?, 
            song_id : row.get(2)?, 
            release : row.get(3)?, 
            artist_id: row.get(4)?, 
            artist_mbid : row.get(5)?, 
            artist_name : row.get(6)?, 
            duration : row.get(7)?, 
            artist_familiarity : row.get(8)?, 
            artist_hotttnesss : row.get(9)?, 
            year : row.get(10)?, 
            track_7digitalid : row.get(11)?, 
            shs_perf : row.get(12)?, 
            shs_work : row.get(13)?,  
           })
       })?;
       let pool = mstore.open();
       let mut mconn = pool.get_conn().expect("error get conn");

       for v in song_iter {
           let mut oo = match v {
               Ok(Song) => Song, 
               Err(e) => return Err(e),
           };
           mstore.insert_song(&mut mconn, oo);
       }
       Ok(())
   }

   pub fn test(&self) -> Result<()> {
       let conn = Connection::open(self.dbfile.clone())?;
       //let sql = String::from("CREATE TABLE songs (track_id text PRIMARY KEY, title text, song_id text, release text, artist_id text, artist_mbid text, artist_name text, duration real, artist_familiarity real, artist_hotttnesss real, year int, track_7digitalid int, shs_perf int, shs_work int);");
       //conn.execute(&sql, params![], )?;

       //let song = Song::new();
       let mut stmt = conn.prepare("select * from songs;")?;
       let song_iter = stmt.query_map(params![], |row| {
           Ok(Song {
            track_id : row.get(0)?, 
            title : row.get(1)?, 
            song_id : row.get(2)?, 
            release : row.get(3)?, 
            artist_id: row.get(4)?, 
            artist_mbid : row.get(5)?, 
            artist_name : row.get(6)?, 
            duration : row.get(7)?, 
            artist_familiarity : row.get(8)?, 
            artist_hotttnesss : row.get(9)?, 
            year : row.get(10)?, 
            track_7digitalid : row.get(11)?, 
            shs_perf : row.get(12)?, 
            shs_work : row.get(13)?,  
           })
       })?;

       for ss in song_iter {
           println!("Found song {:?}", ss);
       }
       Ok(())
    }


}

mod sqlite_store;
mod mysql_store;

fn main() {

    let mstore = mysql_store::MysqlStore::new(
           "Music".to_string(), 
           "192.168.0.250".to_string(), 
            "moonstar".to_string(), 
            "wooag01".to_string() );

    let pool = mstore.open();
    let mut conn = pool.get_conn();



    let infile = "D:/work/data/music_data/track_metadata.db";
    let store = sqlite_store::SqliteStore::new(infile.to_string());
    let sqlite_conn = store.open();
    store.exportMySQL(sqlite_conn, mstore);
    //store.test();
    //println!("Hello, world!");
}

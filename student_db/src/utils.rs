use lazy_static::lazy_static;
use postgres::{Client, NoTls};
use std::sync::{Arc, Mutex};

lazy_static! {
    pub static ref DB_CLIENT: Arc<Mutex<Client>> = Arc::new(Mutex::new(
        Client::connect("postgres://postgres:111111@localhost/student_db", NoTls).unwrap()
    ));
}

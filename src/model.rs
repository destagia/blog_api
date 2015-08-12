use rustc_serialize::json;
use postgres::types::ToSql;
use postgres::rows::Row;

pub trait Postgresable {
    fn insert_values_str() -> String;
    fn get_data(&self) -> String;
    fn table_name() -> String;
    fn parse(row: &Row) -> Self;
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct Account {
    pub id           : String,
    pub name         : String,
    pub password     : String
}

impl Postgresable for Account {

    fn insert_values_str() -> String {
        return "(id, name, password)".to_string();
    }

    fn get_data(&self) -> String {
        format!("({}, {}, {})", self.id, self.name, self.password)
    }

    fn table_name() -> String { "account".to_string() }

    fn parse(row: &Row) -> Self {
        Account { id: row.get(0), name: row.get(1), password: row.get(2) }
    }

}

#[derive(RustcDecodable, RustcEncodable)]
pub struct Article {
    pub id           : String,
    pub title        : String,
    pub body         : String,
    pub timestamp    : String
}

impl Postgresable for Article {

    fn insert_values_str() -> String {
        "(id, title, body, timestamp)".to_string()
    }

    fn get_data(&self) -> String {
        format!("({}, {}, {}, {})", self.id, self.title, self.body, self.timestamp)
    }

    fn table_name() -> String { "article".to_string() }

    fn parse(row: &Row) -> Self {
        Article { id: row.get(0), title: row.get(1), body: row.get(2), timestamp: row.get(3) }
    }

}
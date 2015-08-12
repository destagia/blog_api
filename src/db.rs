
use postgres::{
    Connection, SslMode, ConnectParams, ConnectTarget, UserInfo
};
use model::{Account, Postgresable};

fn connection() -> Box<Connection> {
    let params = ConnectParams {
        target: ConnectTarget::Tcp("localhost".to_string()),
        port: None,
        user: Some(UserInfo {
            user: "shohei".to_string(),
            password: None
        }),
        database: Some("blogapp".to_string()),
        options: vec![],
    };
    Box::new(Connection::connect(params, &SslMode::None).unwrap())
}

fn connect_and<F, T>(action: F) -> T where F : Fn(Box<Connection>) -> T {
    action(connection())
}

pub fn insert<P: Postgresable>(psgre: &P) -> bool {
    let query = format!(
        "insert into {} {} values {}",
        P::table_name(), P::insert_values_str(), psgre.get_data()
    );

    connect_and(|_connect| {
        match _connect.execute(&query, &[]) {
            Ok(_) => true,
            Err(e) => {
                println!("error in `insert_account` : {:?}", e);
                false
            }
        }
    })
}

pub fn select<P: Postgresable>(v: &mut Vec<P>) {
    let query = format!("select * from {}", P::table_name());
    let conn = connection();
    let stmt = conn.prepare(&query);
    match stmt {
        Ok(stmt) => {
            for row in stmt.query(&[]).unwrap() {
                v.push(P::parse(&row));
            }
        },
        Err(e) => println!("error in `select` : {:?}", e)
    }
}
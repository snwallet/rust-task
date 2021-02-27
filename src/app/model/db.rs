use mysql::*;

pub fn pool()->Pool{
    let dsn = String::from("mysql://root:root@192.168.0.220:3306/task");
    let pool = Pool::new(dsn).unwrap();
    return pool;
}

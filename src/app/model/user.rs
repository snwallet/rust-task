#[deny(warnings,unused_imports)]

// use mysql::*;
use mysql::prelude::*;
use crate::app::model::db;
use crate::app::lib::common;
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Eq,Serialize, Deserialize,)]
pub struct User {
    pub id: u64,
    pub name:String,
    pub pwd:String
}



// #[allow(dead_code)]
pub fn insert(user_name:&str, user_pwd:&str) -> u64 {
    let pwd = common::md5(user_pwd);
    let mut conn = db::pool().get_conn().unwrap();
    let ret = "insert into user(name,pwd)
        values(?,?)".with((user_name,pwd)).run(&mut conn).unwrap();
    ret.last_insert_id().unwrap()
}

#[allow(dead_code)]
pub fn update_name(user_name:&str,id:&str) -> u64 {
    let mut conn = db::pool().get_conn().unwrap();
    let ret = "update user set name=? where id=?"
        .with((user_name,id)).run(&mut conn).unwrap();
    println!("ret:{:?}",ret.affected_rows());
    ret.affected_rows()
}

#[allow(dead_code)]
pub fn allow_name(user_name:&str) -> bool {
    let mut conn = db::pool().get_conn().unwrap();
    let ret = "select id,name,pwd from user where is_delete=? and name=?"
        .with((0,user_name))
        .map(&mut conn, |(id,name,pwd)|{
            User{id,name,pwd}
        }).unwrap();
    println!("{:?}",ret);
    if ret.len()>0 { false }else{ true }
}


#[allow(dead_code)]
pub fn delete(id:&str) -> u64 {
    let mut conn = db::pool().get_conn().unwrap();
    let ret = "update user set is_delete=? WHERE id=?".with((1,id)).run(&mut conn).unwrap();
    println!("ret:{:?}",ret.affected_rows());
    ret.affected_rows()
}

#[allow(dead_code)]
pub fn select_all() -> Vec<User> {
    let mut conn = db::pool().get_conn().unwrap();
    let ret = "select id,name,pwd  from user where is_delete=0"
        .map(&mut conn, |(id,name,pwd)|{
            User{id,name,pwd}
        }).unwrap();
    println!("{:?}",ret);
    ret
}

#[allow(dead_code)]
pub fn select_name(name:&str) -> Vec<User> {
    let mut conn = db::pool().get_conn().unwrap();
    let ret = "select id,name,pwd  from user where is_delete=? and name=?"
        .with((0,name)).map(&mut conn, |(id,name,pwd)|{
            User{id,name,pwd}
        }).unwrap();
    println!("{:?}",ret);
    ret
}

#[allow(dead_code)]
pub fn select_id(id:&str) -> Vec<User> {
    let mut conn = db::pool().get_conn().unwrap();
    let ret = "select id,name,pwd from user where is_delete=? and id=?"
        .with((0,id))
        .map(&mut conn, |(id,name,pwd)|{
            User{id,name,pwd}
        }).unwrap();
    println!("{:?}",ret);
    ret
}

#[allow(dead_code)]
pub fn login(name:&str,pwd:&str) -> Vec<User> {
    let real_pwd = common::md5(pwd);
    let mut conn = db::pool().get_conn().unwrap();
    let ret = "select id,name,pwd  from user where is_delete=? and name=? and pwd=?"
        .with((0,name,real_pwd))
        .map(&mut conn, |(id,name,pwd)|{
            User{id,name,pwd}
        }).unwrap();
    println!("{:?}",ret);
    ret
}

#[allow(dead_code)]
pub fn auth_id_pwd(pwd:&str,id:&str) -> Vec<User> {
    let real_pwd = common::md5(pwd);
    let mut conn = db::pool().get_conn().unwrap();
    let ret = "select id,name,pwd  from user where is_delete=? and id=? and pwd=?"
        .with((0,id,real_pwd))
        .map(&mut conn, |(id,name,pwd)|{
            User{id,name,pwd}
        }).unwrap();
    println!("{:?}",ret);
    ret
}

#[allow(dead_code)]
pub fn change_pwd(pwd:&str, id:&str) -> u64 {
    let real_pwd = common::md5(pwd);
    let mut conn = db::pool().get_conn().unwrap();
    let ret = "update user set pwd=? where id=?"
        .with((real_pwd,id)).run(&mut conn).unwrap();
    println!("ret:{:?}",ret.affected_rows());
    ret.affected_rows()
}
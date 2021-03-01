#[deny(warnings,unused_imports)]

// use mysql::*;
use mysql::prelude::*;
use crate::app::model::db;
use crate::app::lib::common;
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Eq,Serialize, Deserialize,)]
pub struct Task {
    pub id: u64,
    pub title:String,
    pub content:String,
    pub create_time:u64,
    pub user_id:u64,
    pub user_name:String,
    pub start_time:u64,
    pub end_time:u64,
}


#[allow(dead_code)]
pub fn insert(title:&str, content:&str) -> u64 {
    let time = common::get_timestamp();
    let mut conn = db::pool().get_conn().unwrap();
    let ret = "insert into task(title,content,create_time)
        values(?,?,?)".with((title,content,time)).run(&mut conn).unwrap();
    ret.last_insert_id().unwrap()
}

#[allow(dead_code)]
pub fn allow_title(title:&str) -> bool {
    let mut conn = db::pool().get_conn().unwrap();
    let ret = "select id,title,content,create_time,user_id,user_name,start_time,end_time from task where is_delete=? and title=?"
        .with((0,title))
        .map(&mut conn, |(id,title,content,create_time,user_id,user_name,start_time,end_time)|{
            Task{id,title,content,create_time,user_id,user_name,start_time,end_time}
        }).unwrap();
    println!("{:?}",ret);
    if ret.len()>0 { false }else{ true }
}

#[allow(dead_code)]
pub fn select_all() -> Vec<Task> {
    let mut conn = db::pool().get_conn().unwrap();
    let ret = "select id,title,content,create_time,user_id,user_name,start_time,end_time  from task where is_delete=0"
        .map(&mut conn, |(id,title,content,create_time,user_id,user_name,start_time,end_time)|{
            Task{id,title,content,create_time,user_id,user_name,start_time,end_time}
        }).unwrap();
    println!("{:?}",ret);
    ret
}

#[allow(dead_code)]
pub fn select_title(title:&str) -> Vec<Task> {
    let mut conn = db::pool().get_conn().unwrap();
    let ret = "select id,title,content,create_time,user_id,user_name,start_time,end_time  from task where is_delete=? and title=?"
        .with((0,title)).map(&mut conn, |(id,title,content,create_time,user_id,user_name,start_time,end_time)|{
        Task{id,title,content,create_time,user_id,user_name,start_time,end_time}
    }).unwrap();
    println!("{:?}",ret);
    ret
}

#[allow(dead_code)]
pub fn delete(id:&str) -> u64 {
    let mut conn = db::pool().get_conn().unwrap();
    let ret = "update task set is_delete=? WHERE id=?".with((1,id)).run(&mut conn).unwrap();
    println!("ret:{:?}",ret.affected_rows());
    ret.affected_rows()
}

#[allow(dead_code)]
pub fn update(title:&str,content:&str,id:&str) -> u64 {
    let mut conn = db::pool().get_conn().unwrap();
    let ret = "update task set title=?,content=? where id=?"
        .with((title,content,id)).run(&mut conn).unwrap();
    println!("ret:{:?}",ret.affected_rows());
    ret.affected_rows()
}

#[allow(dead_code)]
pub fn select_id(id:&str) -> Vec<Task> {
    let mut conn = db::pool().get_conn().unwrap();
    let ret = "select id,title,content,create_time,user_id,user_name,start_time,end_time from task where is_delete=? and id=?"
        .with((0,id))
        .map(&mut conn, |(id,title,content,create_time,user_id,user_name,start_time,end_time)|{
            Task{id,title,content,create_time,user_id,user_name,start_time,end_time}
        }).unwrap();
    println!("{:?}",ret);
    ret
}

#[allow(dead_code)]
pub fn select_user_id(id:&str) -> Vec<Task> {
    let mut conn = db::pool().get_conn().unwrap();
    let ret = "select id,title,content,create_time,user_id,user_name,start_time,end_time from task where is_delete=? and user_id=?"
        .with((0,id))
        .map(&mut conn, |(id,title,content,create_time,user_id,user_name,start_time,end_time)|{
            Task{id,title,content,create_time,user_id,user_name,start_time,end_time}
        }).unwrap();
    println!("{:?}",ret);
    ret
}

#[allow(dead_code)]
pub fn task_add_user(user_id:&str, user_name:&str,id:&str) -> u64 {
    let time = common::get_timestamp();
    let mut conn = db::pool().get_conn().unwrap();
    let ret = "update task set user_id=?,user_name=?,start_time=? where id=?"
        .with((user_id,user_name,time,id))
        .run(&mut conn).unwrap();
    println!("{:?}",ret);
    ret.last_insert_id().unwrap()
}


#[allow(dead_code)]
pub fn task_end_user(id:&str) -> u64 {
    let time = common::get_timestamp();
    let mut conn = db::pool().get_conn().unwrap();
    let ret = "update task set end_time=? where id=?"
        .with((time,id))
        .run(&mut conn).unwrap();
    println!("{:?}",ret);
    ret.last_insert_id().unwrap()
}
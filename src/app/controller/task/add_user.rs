
use crate::app::lib::json_res::JsonRes;
use hyper::{Request, Body, Response};
use crate::app::lib::param;
use crate::app::model::task;

pub async fn main(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let params = param::post_param(req).await?;
    //insert(store_title:String, store_name:String, store_pwd:String, store_desc:String)
    let is_1 = params.contains_key("user_id");
    let is_2 = params.contains_key("user_name");
    let is_3 = params.contains_key("id");

    if is_1 && is_2 && is_3  {
        let user_id = params.get("user_id").unwrap();
        let user_name = params.get("user_name").unwrap();
        let id = params.get("id").unwrap();
        let insert_id = task::task_add_user(user_id,user_name,id);
        JsonRes::new(0,"success".to_string(),insert_id)

    }else{
        JsonRes::new(-1,"param error".to_string(),"")
    }
}

use crate::app::lib::json_res::JsonRes;
use hyper::{Request, Body, Response};
use crate::app::lib::param;
use crate::app::model::task;

pub async fn main(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let params = param::post_param(req).await?;
    //insert(store_title:String, store_name:String, store_pwd:String, store_desc:String)
    let is_1 = params.contains_key("title");
    let is_2 = params.contains_key("content");

    if is_1 && is_2  {
        let title = params.get("title").unwrap();
        let content = params.get("content").unwrap();
        //检查店铺名称 登录用户名是否重复
        let title_bool = task::allow_title(title);
        match title_bool {
            true=>{
                let insert_id = task::insert(title,content);
                JsonRes::new(0,"success".to_string(),insert_id)
            },

            _=>JsonRes::new(-2,"title is exist".to_string(),"")

        }
    }else{
        JsonRes::new(-1,"param error".to_string(),"")
    }
}
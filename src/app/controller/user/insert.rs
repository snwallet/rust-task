
use crate::app::lib::json_res::JsonRes;
use hyper::{Request, Body, Response};
use crate::app::lib::param;
use crate::app::model::user;

pub async fn main(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let params = param::post_param(req).await?;
    //insert(store_title:String, store_name:String, store_pwd:String, store_desc:String)
    let is_1 = params.contains_key("name");
    let is_2 = params.contains_key("pwd");

    if is_1 && is_2  {
        let name = params.get("name").unwrap();
        let pwd = params.get("pwd").unwrap();
        //检查店铺名称 登录用户名是否重复
        let name_bool = user::allow_name(name);
        match name_bool {
            true=>{
                let insert_id = user::insert(name,pwd);
                JsonRes::new(0,"success".to_string(),insert_id)
            },

            _=>JsonRes::new(-2,"store title and name is exist".to_string(),"")

        }
    }else{
        JsonRes::new(-1,"param error".to_string(),"")
    }
}

use crate::app::lib::json_res::JsonRes;
use hyper::{Request, Body, Response};
use crate::app::lib::param;
use crate::app::model::user;

pub async fn main(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let params = param::post_param(req).await?;
    //insert(store_title:String, store_name:String, store_pwd:String, store_desc:String)
    let is_1 = params.contains_key("name");
    let is_2 = params.contains_key("id");

    if is_1 && is_2 {
        let name = params.get("name").unwrap();
        let id = params.get("id").unwrap();

        match user::select_id(id).get(0) {
            Some(s)=>{
                println!("s:{:?}",*name);
                println!("s:{:?}",s.name);
                let name_bool = s.name==*name || user::allow_name(name);

                match name_bool {
                    true => {
                        let res = user::update_name(name, id);
                        JsonRes::new(0, "success".to_string(), res)
                    },
                    _ => JsonRes::new(-3, " name is registered".to_string(), "")
                }
            },
            None=>JsonRes::new(-2,"id error".to_string(),"")
        }
    }else{
        JsonRes::new(-1,"param error".to_string(),"")
    }
}
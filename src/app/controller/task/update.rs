
use crate::app::lib::json_res::JsonRes;
use hyper::{Request, Body, Response};
use crate::app::lib::param;
use crate::app::model::task;

pub async fn main(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let params = param::post_param(req).await?;
    //insert(store_title:String, store_name:String, store_pwd:String, store_desc:String)
    let is_1 = params.contains_key("title");
    let is_2 = params.contains_key("content");
    let is_3 = params.contains_key("id");

    if is_1 && is_2 && is_3 {
        let title = params.get("title").unwrap();
        let content = params.get("content").unwrap();
        let id = params.get("id").unwrap();

        match task::select_id(id).get(0) {
            Some(s)=>{
                println!("s:{:?}",*title);
                println!("s:{:?}",s.title);
                let title_bool = s.title==*title || task::allow_title(title);

                match title_bool {
                    true => {
                        let res = task::update(title, content,id);
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
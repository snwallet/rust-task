
use crate::app::lib::json_res::JsonRes;
use hyper::{Request, Body, Response};
use crate::app::model::task;
use crate::app::lib::param;

pub async fn main(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let params = param::post_param(req).await?;
    let is_1 = params.contains_key("user_id");

    if is_1 {
        let user_id = params.get("user_id").unwrap();
        let data = task::select_user_id(user_id);
        JsonRes::new(0,"success".to_string(),data)
    }else{
        let data = task::select_all();
        JsonRes::new(0,"success".to_string(),data)
    }
}
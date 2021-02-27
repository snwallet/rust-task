
use crate::app::lib::json_res::JsonRes;
use hyper::{Request, Body, Response};
use crate::app::model::user;
use crate::app::lib::param;

pub async fn main(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let params = param::post_param(req).await?;
    let is_1 = params.contains_key("name");

    if is_1 {
        let name = params.get("name").unwrap();
        let data = user::select_name(name);
        JsonRes::new(0,"success".to_string(),data)
    }else{
        let data = user::select_all();
        JsonRes::new(0,"success".to_string(),data)
    }
}
use super::json::structure::ResponseHandler;

pub async fn get(url: String) -> Vec<ResponseHandler> {
    let res = reqwest::get(url.as_str()).await.unwrap();
    let txt = res.json::<Vec<ResponseHandler>>().await.unwrap();

    return txt;
}
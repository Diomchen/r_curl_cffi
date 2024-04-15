//! Module:         models
//! Description:    Save Instance Struct

/// Contains information the server sends.
///
///
#[derive(Clone)]
pub struct Response{
    curl : String,
    request : String,
    url : String,
    content : String,
    status_code : String,
    reason : String,
    ok : String,
    headers : String,
    cookies : String,
    elapsed : String,
    default_encoding : String,
    redirect_count : String,
    redirect_url : String,
    http_version : String,
    history : String,
    infos : String,
    queue : String,
    stream_task : String,
    astream_task : String,
    quit_now : String,
}

impl Response{
    pub fn init(){

    }
}
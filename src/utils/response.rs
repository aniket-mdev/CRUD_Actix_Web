use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseBuilder<T> {
    pub status:bool,
    pub error:String,
    pub msg:String,
    pub data: Option<T>,
}
#[allow(non_snake_case)]
impl<T>  ResponseBuilder <T> {
    pub fn BuildFailedResponse(err_msg:String) -> ResponseBuilder<T> {
        ResponseBuilder {
            status:false,
            error:String::from("Bad Request"),
            msg:err_msg,
            data:None
        }   
    }

    pub fn BuildSuccessResponse(succ_msg:String, data:Option<T>) -> ResponseBuilder<T> {
        ResponseBuilder {
            status:true,
            error:String::from(""),
            msg:succ_msg,
            data
        }
    }
}
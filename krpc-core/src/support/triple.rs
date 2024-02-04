#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TripleRequestWrapper {
    /// hessian4
    /// json
    #[prost(string, tag = "1")]
    pub serialize_type: ::prost::alloc::string::String,
    #[prost(bytes = "vec", repeated, tag = "2")]
    pub args: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, repeated, tag = "3")]
    pub arg_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TripleResponseWrapper {
    #[prost(string, tag = "1")]
    pub serialize_type: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "3")]
    pub r#type: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TripleExceptionWrapper {
    #[prost(string, tag = "1")]
    pub language: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub serialization: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub class_name: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "4")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}

impl TripleRequestWrapper {
    pub fn get_request(strs: Vec<String>) -> Self {
        let mut trip = TripleRequestWrapper::default();
        trip.serialize_type = "fastjson2".to_string();
        trip.args = vec![];
        for str in strs {
            trip.args.push(str.as_bytes().to_vec());
        }
        trip.arg_types = vec!["org.apache.dubbo.springboot.demo.ReqDto".to_string()];
        return trip;
    }

    pub fn get_req(&self)-> Vec<String> {
        let mut res = vec![];
        for str in &self.args {
            res.push(String::from_utf8(str.clone()).unwrap());
        }
        return res;
    }
}
use http_body::Frame;
use std::marker::PhantomData;

use super::BodyCodec;

pub struct JsonBodyCodec<D> {
    _d: PhantomData<D>,
}

impl<D> JsonBodyCodec<D> {
    pub fn new() -> Self {
        Self { _d: PhantomData }
    }
}

impl<D> BodyCodec<D> for JsonBodyCodec<D>
where
    D: bytes::Buf,
{
    type DecodeType = Vec<String>;

    type EncodeType = Vec<String>;

    fn decode(&self, body: Vec<Frame<D>>) -> Result<Vec<String>, crate::Error> {
        let data = if body.is_empty() || body[0].is_trailers() {
            return Err("receive frame err".into());
        } else {
            body[0].data_ref().unwrap().chunk()
        };
        Ok(if data.starts_with(b"[") {
            match serde_json::from_slice(&data) {
                Ok(req) => req,
                Err(err) => return Err(err.into()),
            }
        } else {
            let mut pre = String::new();
            if !data.starts_with(b"\"") {
                pre.push_str("\"\"");
                pre.insert_str(1, std::str::from_utf8(data).unwrap());
            } else {
                pre.push_str(std::str::from_utf8(data).unwrap());
            }
            vec![pre]
        })
    }

    fn encode(&self, mut res: Vec<String>) -> Result<bytes::Bytes, crate::Error> {
        if res.is_empty() {
            return Err("encode err res is empty".into());
        }
        let res = if res.len() == 1 {
            res.remove(0)
        } else {
            serde_json::to_string(&res)?
        };
        Ok(bytes::Bytes::from(res))
    }
}

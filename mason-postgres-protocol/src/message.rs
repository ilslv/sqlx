use crate::{Decode, Encode, ReadyForQuery, Response};
use byteorder::{BigEndian, ReadBytesExt};
use bytes::Bytes;
use std::io::{self, Cursor};

#[derive(Debug)]
pub enum Message {
    ReadyForQuery(ReadyForQuery),
    Response(Response),
}

impl Encode for Message {
    fn size_hint(&self) -> usize {
        match self {
            Message::ReadyForQuery(body) => body.size_hint(),
            Message::Response(body) => body.size_hint(),
        }
    }

    fn encode(&self, buf: &mut Vec<u8>) -> io::Result<()> {
        match self {
            Message::ReadyForQuery(body) => body.encode(buf),
            Message::Response(body) => body.encode(buf),
        }
    }
}

impl Decode for Message {
    fn decode(src: Bytes) -> io::Result<Self>
    where
        Self: Sized,
    {
        let mut buf = Cursor::new(&src);

        let token = buf.read_u8()?;
        let len = buf.read_u32::<BigEndian>()? as usize;
        let pos = buf.position() as usize;

        // `len` includes the size of the length u32
        let src = src.slice(pos, pos + len - 4);

        Ok(match token {
            b'N' | b'E' => Message::Response(Response::decode(src)?),
            b'Z' => Message::ReadyForQuery(ReadyForQuery::decode(src)?),

            _ => unimplemented!("decode not implemented for token: {}", token as char),
        })
    }
}

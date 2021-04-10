use crate::packetizer::Payloader;
use bytes::BytesMut;

mod g722_test;

pub struct G722Payloader;

impl Payloader for G722Payloader {
    fn payload(&self, mtu: u16, mut payload: bytes::Bytes) -> Vec<bytes::Bytes> {
        let mut payloads = vec![];
        if payload.is_empty() || mtu == 0 {
            return payloads;
        }

        while payload.len() > mtu as usize {
            let mut o = BytesMut::with_capacity(mtu as usize);
            o.resize(mtu as usize, 0u8);
            o.copy_from_slice(&payload[..mtu as usize]);
            payload = payload.split_off(mtu as usize);
            payloads.push(o.into())
        }

        let mut o = BytesMut::with_capacity(mtu as usize);
        o.resize(payload.len(), 0u8);
        o.copy_from_slice(&payload);
        payloads.push(o.into());

        payloads
    }
}

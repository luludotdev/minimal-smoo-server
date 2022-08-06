use bytes::{Buf, BytesMut};
use color_eyre::Report;
use tokio_util::codec::{Decoder, Encoder};

use super::header::{Packet, PartialPacket};
use super::traits::PacketBytes;

pub struct PacketCodec;

impl Decoder for PacketCodec {
    type Item = Packet;
    type Error = Report;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if buf.remaining() < Packet::buf_size() {
            buf.reserve(Packet::buf_size());

            return Ok(None);
        }

        let partial = PartialPacket::from_bytes(buf)?;
        let body_len = partial.body_length as usize;

        // Catch obviously wrong packets
        if body_len > 1024 {
            buf.clear();
            buf.reserve(Packet::buf_size());

            return Ok(None);
        }

        if buf.remaining() < body_len {
            buf.reserve(body_len);

            return Ok(None);
        }

        let mut body = buf.split_to(body_len);
        let packet = partial.upgrade(&mut body)?;
        buf.reserve(Packet::buf_size());

        Ok(Some(packet))
    }
}

impl Encoder<Packet> for PacketCodec {
    type Error = Report;

    #[inline]
    fn encode(&mut self, item: Packet, buf: &mut BytesMut) -> Result<(), Self::Error> {
        item.write_bytes(buf);

        Ok(())
    }
}

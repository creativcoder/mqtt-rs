//! PUBREC

use std::io::{Read, Write};

use control::{FixedHeader, PacketType, ControlType};
use control::variable_header::PacketIdentifier;
use packet::{Packet, PacketError};
use {Encodable, Decodable};

/// `PUBREC` packet
#[derive(Debug, Eq, PartialEq)]
pub struct PubrecPacket {
    fixed_header: FixedHeader,
    packet_identifier: PacketIdentifier,
    payload: (),
}

impl PubrecPacket {
    pub fn new(pkid: u16) -> PubrecPacket {
        PubrecPacket {
            fixed_header: FixedHeader::new(PacketType::with_default(ControlType::PublishReceived), 2),
            packet_identifier: PacketIdentifier(pkid),
            payload: (),
        }
    }

    pub fn packet_identifier(&self) -> u16 {
        self.packet_identifier.0
    }

    pub fn set_packet_identifier(&mut self, pkid: u16) {
        self.packet_identifier.0 = pkid;
    }
}

impl<'a> Packet<'a> for PubrecPacket {
    type Payload = ();

    fn fixed_header(&self) -> &FixedHeader {
        &self.fixed_header
    }

    fn payload(&self) -> &Self::Payload {
        &self.payload
    }

    fn encode_variable_headers<W: Write>(&self, writer: &mut W) -> Result<(), PacketError<'a, Self>> {
        try!(self.packet_identifier.encode(writer));

        Ok(())
    }

    fn encoded_variable_headers_length(&self) -> u32 {
        self.packet_identifier.encoded_length()
    }

    fn decode_packet<R: Read>(reader: &mut R, fixed_header: FixedHeader) -> Result<Self, PacketError<'a, Self>> {
        let packet_identifier: PacketIdentifier = try!(PacketIdentifier::decode(reader));
        Ok(PubrecPacket {
               fixed_header: fixed_header,
               packet_identifier: packet_identifier,
               payload: (),
           })
    }
}

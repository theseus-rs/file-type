use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855588: FileFormat = FileFormat {
    id: 105_855_588,
    puid: "wikidata/105855588",
    name: "OpenVPN profile (var.1)",
    extensions: &["ovpn"],
    media_types: &["application/x-openvpn-profile"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x63, 0x6C, 0x69, 0x65, 0x6E, 0x74])],
            },
        }],
    }],
    related_formats: &[],
};

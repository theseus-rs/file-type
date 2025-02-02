use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855608: FileFormat = FileFormat {
    id: 105_855_608,
    source_type: SourceType::Wikidata,
    name: "OpenVPN profile (var.2)",
    extensions: &["ovpn"],
    media_types: &["application/x-openvpn-profile"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x64, 0x65, 0x76])],
            },
        }],
    }],
    related_formats: &[],
};

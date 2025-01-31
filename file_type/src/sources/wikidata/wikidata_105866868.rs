use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866868: FileFormat = FileFormat {
    id: 105_866_868,
    puid: "wikidata/105866868",
    name: "Cisco VPN Profile Configuration File",
    extensions: &["pcf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x5B, 0x6D, 0x61, 0x69, 0x6E, 0x5D])],
            },
        }],
    }],
    related_formats: &[],
};

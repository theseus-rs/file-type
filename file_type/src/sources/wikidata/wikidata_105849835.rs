use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849835: FileFormat = FileFormat {
    id: 105_849_835,
    puid: "wikidata/105849835",
    name: "MS Flight Simulator aircraft configuration file",
    extensions: &["cfg"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x66, 0x6C, 0x74, 0x73, 0x69, 0x6D, 0x2E, 0x30, 0x5D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

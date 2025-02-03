use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856030: FileFormat = FileFormat {
    id: 105_856_030,
    source_type: SourceType::Wikidata,
    name: "IBM Document Content Architecture / Revisable Form Text",
    extensions: &["dca", "rft"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x05, 0xE1, 0x03, 0x00, 0x00])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x05, 0xE1, 0x03, 0x00, 0x00])],
                },
            }],
        },
    ],
    related_formats: &[],
};

use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105854333: FileFormat = FileFormat {
    id: 105_854_333,
    source_type: SourceType::Wikidata,
    name: "Authorware Packaged file (with runtime)",
    extensions: &["a4r", "a5r"],
    media_types: &[],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x43, 0x52, 0x53])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x43, 0x52, 0x53])],
                },
            }],
        },
    ],
    related_formats: &[],
};

use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855618: FileFormat = FileFormat {
    id: 105_855_618,
    source_type: SourceType::Wikidata,
    name: "Openlab Raw Format",
    extensions: &["olr", "olrw"],
    media_types: &[],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4F, 0x4C, 0x52, 0x57])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4F, 0x4C, 0x52, 0x57])],
                },
            }],
        },
    ],
    related_formats: &[],
};

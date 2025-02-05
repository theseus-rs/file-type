use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1850: FileFormat = FileFormat {
    id: 1_850,
    source_type: SourceType::Pronom,
    name: "Q&A Word Processor Document",
    extensions: &[],
    media_types: &[],
    signatures: &[
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x54, 0x42, 0x57, 0x50, 0x00])],
                },
            }],
        },
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x54, 0x42, 0x54, 0x58, 0x00])],
                },
            }],
        },
    ],
    related_formats: &[],
};

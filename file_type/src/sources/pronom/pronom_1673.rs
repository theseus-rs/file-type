use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1673: FileFormat = FileFormat {
    id: 1_673,
    source_type: SourceType::Pronom,
    name: "CDX Internet Archive Index",
    extensions: &["cdx"],
    media_types: &[],
    signatures: &[
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x20, 0x43, 0x44, 0x58, 0x20])],
                },
            }],
        },
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x44, 0x58, 0x20])],
                },
            }],
        },
    ],
    related_formats: &[],
};

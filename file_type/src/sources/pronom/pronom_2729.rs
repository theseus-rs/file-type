use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2729: FileFormat = FileFormat {
    id: 2_729,
    source_type: SourceType::Pronom,
    name: "Maptech BSB Documentation File",
    extensions: &["bsb", "kap"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x45, 0x44, 0x2F])],
                },
            },
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4E, 0x54, 0x4D, 0x2F])],
                },
            },
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x56, 0x45, 0x52, 0x2F])],
                },
            },
        ],
    }],
    related_formats: &[],
};

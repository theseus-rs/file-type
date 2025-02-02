use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1492: FileFormat = FileFormat {
    id: 1_492,
    source_type: SourceType::Pronom,
    name: "Mach-O",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xFE, 0xED, 0xFA, 0xCF])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xCF, 0xFA, 0xED, 0xFE])],
                },
            }],
        },
    ],
    related_formats: &[],
};

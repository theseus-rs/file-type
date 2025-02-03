use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1144: FileFormat = FileFormat {
    id: 1_144,
    source_type: SourceType::Pronom,
    name: "PocketMobi (Palm Resource) File",
    extensions: &["mobi", "prc"],
    media_types: &[],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(60),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x42, 0x4F, 0x4F, 0x4B, 0x4D, 0x4F, 0x42, 0x49,
                    ])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(60),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x54, 0x45, 0x58, 0x74, 0x52, 0x45, 0x41, 0x64,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::HasLowerPriorityThan,
        id: 2_798,
    }],
};

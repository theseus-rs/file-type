use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_396: FileFormat = FileFormat {
    id: 1_144,
    puid: "fmt/396",
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
        id: 2_798,
        relationship_type: RelationshipType::HasLowerPriorityThan,
    }],
};

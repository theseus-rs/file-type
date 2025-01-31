use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1176: FileFormat = FileFormat {
    id: 1_986,
    puid: "fmt/1176",
    name: "Nullsoft Streaming Video",
    extensions: &["nsv"],
    media_types: &[],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x4E, 0x53, 0x56, 0x66]),
                        Token::WildcardCountRange(24, 16_384),
                        Token::Literal(&[0x4E, 0x53, 0x56, 0x73]),
                    ],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4E, 0x53, 0x56, 0x73])],
                },
            }],
        },
    ],
    related_formats: &[RelatedFormat {
        id: 687,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};

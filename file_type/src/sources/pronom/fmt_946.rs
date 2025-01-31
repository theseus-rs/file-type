use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_946: FileFormat = FileFormat {
    id: 1_751,
    puid: "fmt/946",
    name: "Ogg Opus Codec Compressed Multimedia File",
    extensions: &["ogg", "opus"],
    media_types: &["audio/ogg", "audio/opus"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x4F, 0x67, 0x67, 0x53, 0x00, 0x02]),
                    Token::WildcardCount(22),
                    Token::Literal(&[0x4F, 0x70, 0x75, 0x73, 0x48, 0x65, 0x61, 0x64]),
                    Token::WildcardCount(11),
                    Token::Literal(&[0x4F, 0x67, 0x67, 0x53]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 1_749,
            relationship_type: RelationshipType::CanBeContainedBy,
        },
        RelatedFormat {
            id: 1_749,
            relationship_type: RelationshipType::HasPriorityOver,
        },
    ],
};

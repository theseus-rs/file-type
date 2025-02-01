use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_203: FileFormat = FileFormat {
    id: 929,
    puid: "fmt/203",
    name: "Ogg Vorbis Codec Compressed Multimedia File",
    extensions: &["ogg"],
    media_types: &["audio/ogg"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x4F, 0x67, 0x67, 0x53, 0x00, 0x02]),
                    Token::WildcardCount(23),
                    Token::Literal(&[0x76, 0x6F, 0x72, 0x62, 0x69, 0x73, 0x00, 0x00, 0x00, 0x00]),
                    Token::WildcardCount(19),
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

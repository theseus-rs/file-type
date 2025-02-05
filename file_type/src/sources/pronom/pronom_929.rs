use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_929: FileFormat = FileFormat {
    id: 929,
    source_type: SourceType::Pronom,
    name: "Ogg Vorbis Codec Compressed Multimedia File",
    extensions: &["ogg"],
    media_types: &["audio/ogg"],
    signatures: &[Signature {
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
            relationship_type: RelationshipType::CanBeContainedBy,
            id: 1_749,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 1_749,
        },
    ],
};

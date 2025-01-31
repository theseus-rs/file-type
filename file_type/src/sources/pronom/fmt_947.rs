use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_947: FileFormat = FileFormat {
    id: 1_752,
    puid: "fmt/947",
    name: "Ogg FLAC Compressed Multimedia File",
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
                    Token::Literal(&[0x46, 0x4C, 0x41, 0x43]),
                    Token::WildcardCount(46),
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

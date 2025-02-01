use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_948: FileFormat = FileFormat {
    id: 1_753,
    puid: "fmt/948",
    name: "Ogg Speex Codec Multimedia File",
    extensions: &["ogg", "spx"],
    media_types: &["audio/ogg", "audio/speex"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x4F, 0x67, 0x67, 0x53, 0x00, 0x02]),
                    Token::WildcardCount(22),
                    Token::Literal(&[0x53, 0x70, 0x65, 0x65, 0x78, 0x20, 0x20, 0x20]),
                    Token::WildcardCount(72),
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

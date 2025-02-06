use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_306: FileFormat = FileFormat {
    id: 306,
    source_type: SourceType::Pronom,
    name: "Microsoft Powerpoint Packaged Presentation",
    extensions: &["ppz"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x4D, 0x53, 0x43, 0x46]),
                    Token::WildcardCount(20),
                    Token::Literal(&[0x03, 0x01]),
                    Token::AnyWildcard,
                    Token::Literal(&[0x2E]),
                    Token::Any(&[
                        &[Token::Literal(&[0x50, 0x50, 0x54])],
                        &[Token::Literal(&[0x70, 0x70, 0x74])],
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::HasPriorityOver,
        id: 801,
    }],
};

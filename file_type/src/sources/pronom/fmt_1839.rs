use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1839: FileFormat = FileFormat {
    id: 2_691,
    puid: "fmt/1839",
    name: "Microsoft Publisher Packaged Document",
    extensions: &["puz"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
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
                        &[Token::Literal(&[0x50, 0x55, 0x42])],
                        &[Token::Literal(&[0x70, 0x75, 0x62])],
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 801,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};

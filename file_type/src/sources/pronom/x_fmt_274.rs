use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_274: FileFormat = FileFormat {
    id: 406,
    puid: "x-fmt/274",
    name: "Microsoft Word for MS-DOS Document",
    extensions: &["doc"],
    media_types: &["application/msword"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[
                        0x31, 0xBE, 0x00, 0x00, 0x00, 0xAB, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                        0x00, 0x00,
                    ]),
                    Token::WildcardCount(82),
                    Token::Literal(&[0x00, 0x00]),
                    Token::WildcardCount(18),
                    Token::Literal(&[0x00]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 407,
        relationship_type: RelationshipType::IsPreviousVersionOf,
    }],
};

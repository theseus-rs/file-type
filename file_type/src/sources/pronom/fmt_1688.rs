use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1688: FileFormat = FileFormat {
    id: 2_524,
    puid: "fmt/1688",
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
                    Token::Any(&[&[Token::Literal(&[0x08])], &[Token::Literal(&[0x09])]]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 408,
        relationship_type: RelationshipType::IsSubsequentVersionOf,
    }],
};

use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1656: FileFormat = FileFormat {
    id: 2_483,
    puid: "fmt/1656",
    name: "Microsoft Help Contents File",
    extensions: &["cnt"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x3A, 0x42, 0x61, 0x73, 0x65]),
                    Token::WildcardCountRange(1, 259),
                    Token::Literal(&[0x2E]),
                    Token::Any(&[
                        &[Token::Literal(&[0x48, 0x4C, 0x50])],
                        &[Token::Literal(&[0x68, 0x6C, 0x70])],
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[],
};

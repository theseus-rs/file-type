use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_232: FileFormat = FileFormat {
    id: 324,
    puid: "x-fmt/232",
    name: "Microsoft Project Export File",
    extensions: &["mpx"],
    media_types: &["application/x-project"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x4D, 0x50, 0x58]),
                    Token::Any(&[&[Token::Literal(&[0x2C])], &[Token::Literal(&[0x3B])]]),
                    Token::WildcardCountRange(1, 50),
                    Token::Any(&[&[Token::Literal(&[0x2C])], &[Token::Literal(&[0x3B])]]),
                    Token::Literal(&[0x31, 0x2E, 0x30]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 1_088,
        relationship_type: RelationshipType::IsPreviousVersionOf,
    }],
};

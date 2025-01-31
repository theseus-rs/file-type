use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_405: FileFormat = FileFormat {
    id: 1_153,
    puid: "fmt/405",
    name: "Portable Any Map",
    extensions: &["pam"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x50, 0x37]),
                    Token::AnyWildcard,
                    Token::Literal(&[0x57, 0x49, 0x44, 0x54, 0x48]),
                    Token::WildcardCountRange(3, 256),
                    Token::Literal(&[0x45, 0x4E, 0x44, 0x48, 0x44, 0x52]),
                ],
            },
        }],
    }],
    related_formats: &[],
};

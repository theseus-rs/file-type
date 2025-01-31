use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_307: FileFormat = FileFormat {
    id: 1_052,
    puid: "fmt/307",
    name: "Quicken Interchange Format",
    extensions: &["qif"],
    media_types: &["application/qif"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x21, 0x54, 0x79, 0x70, 0x65, 0x3A]),
                    Token::AnyWildcard,
                    Token::Literal(&[0x5E]),
                ],
            },
        }],
    }],
    related_formats: &[],
};

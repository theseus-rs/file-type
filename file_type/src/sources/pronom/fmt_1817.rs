use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1817: FileFormat = FileFormat {
    id: 2_668,
    puid: "fmt/1817",
    name: "Direct Stream Digital Stream File",
    extensions: &["dsf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x44, 0x53, 0x44, 0x20]),
                    Token::WildcardCount(24),
                    Token::Literal(&[0x66, 0x6D, 0x74, 0x20]),
                ],
            },
        }],
    }],
    related_formats: &[],
};

use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_436: FileFormat = FileFormat {
    id: 843,
    puid: "x-fmt/436",
    name: "CATIA Model",
    extensions: &["mod", "model"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(80),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x43, 0x41, 0x54, 0x49, 0x41, 0x20, 0x20, 0x20]),
                    Token::AnyWildcard,
                    Token::Literal(&[
                        0x43, 0x41, 0x54, 0x49, 0x41, 0x20, 0x53, 0x4F, 0x4C, 0x55, 0x54, 0x49,
                        0x4F, 0x4E, 0x53, 0x20, 0x56, 0x34,
                    ]),
                    Token::WildcardCount(6),
                    Token::Literal(&[0x52, 0x45, 0x4C, 0x45, 0x41, 0x53, 0x45, 0x20]),
                ],
            },
        }],
    }],
    related_formats: &[],
};

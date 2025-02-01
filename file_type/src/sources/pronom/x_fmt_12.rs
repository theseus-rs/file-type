use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_12: FileFormat = FileFormat {
    id: 28,
    puid: "x-fmt/12",
    name: "Write for Windows Document",
    extensions: &["wri"],
    media_types: &[],
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
                    Token::NotLiteral(&[0x00, 0x00]),
                ],
            },
        }],
    }],
    related_formats: &[],
};

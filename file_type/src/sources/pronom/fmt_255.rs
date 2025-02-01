use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_255: FileFormat = FileFormat {
    id: 993,
    puid: "fmt/255",
    name: "DjVu File Format",
    extensions: &["djvu", "djv"],
    media_types: &["image/vnd.djvu", "image/x-djvu"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x41, 0x54, 0x26, 0x54, 0x46, 0x4F, 0x52, 0x4D]),
                    Token::WildcardCount(4),
                    Token::Any(&[
                        &[Token::Literal(&[
                            0x44, 0x4A, 0x56, 0x4D, 0x44, 0x49, 0x52, 0x4D,
                        ])],
                        &[Token::Literal(&[
                            0x44, 0x4A, 0x56, 0x55, 0x49, 0x4E, 0x46, 0x4F,
                        ])],
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[],
};

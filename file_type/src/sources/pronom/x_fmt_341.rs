use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_341: FileFormat = FileFormat {
    id: 505,
    puid: "x-fmt/341",
    name: "Macromedia Director",
    extensions: &["dir", "dxr"],
    media_types: &["application/x-director"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x52, 0x49, 0x46, 0x58]),
                    Token::WildcardCount(4),
                    Token::Literal(&[
                        0x4D, 0x56, 0x39, 0x33, 0x69, 0x6D, 0x61, 0x70, 0x00, 0x00, 0x00, 0x18,
                        0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x2C,
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[],
};

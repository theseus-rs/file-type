use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_218: FileFormat = FileFormat {
    id: 309,
    puid: "x-fmt/218",
    name: "ESRI Arc/Info Binary Grid",
    extensions: &["adf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x00, 0x00, 0x27, 0x0A, 0xFF, 0xFF]),
                    Token::Any(&[
                        &[Token::Literal(&[0xFC, 0x14])],
                        &[Token::Literal(&[0xFB, 0xF8])],
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[],
};

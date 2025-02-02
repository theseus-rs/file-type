use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_309: FileFormat = FileFormat {
    id: 309,
    source_type: SourceType::Pronom,
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

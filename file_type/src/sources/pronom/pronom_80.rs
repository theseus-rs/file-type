use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_80: FileFormat = FileFormat {
    id: 80,
    source_type: SourceType::Pronom,
    name: "AutoCAD Design Web Format",
    extensions: &["dwf"],
    media_types: &[
        "application/dwf",
        "application/x-dwf",
        "drawing/x-dwf",
        "image/vnd.dwf",
        "image/x-dwf",
        "model/vnd.dwf",
    ],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x28, 0x44, 0x57, 0x46, 0x20, 0x56, 0x30]),
                    Token::SingleWildcard,
                    Token::Literal(&[0x2E]),
                    Token::SingleWildcard,
                    Token::SingleWildcard,
                    Token::Literal(&[0x29]),
                ],
            },
        }],
    }],
    related_formats: &[],
};

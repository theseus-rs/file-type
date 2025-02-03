use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1879: FileFormat = FileFormat {
    id: 1_879,
    source_type: SourceType::Pronom,
    name: "FileMaker Pro Database",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(525),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[
                        0x48, 0x42, 0x41, 0x4D, 0x32, 0x30, 0x30, 0x31, 0x4D, 0x41, 0x59, 0x38,
                        0x34,
                    ]),
                    Token::AnyWildcard,
                    Token::Literal(&[
                        0x45, 0x07, 0x50, 0x72, 0x6F, 0x20, 0x31, 0x2E, 0x30, 0x46, 0x21,
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[],
};

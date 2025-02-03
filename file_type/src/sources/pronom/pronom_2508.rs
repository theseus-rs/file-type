use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2508: FileFormat = FileFormat {
    id: 2_508,
    source_type: SourceType::Pronom,
    name: "Linux/i386 Binary Executable File ZMAGIC",
    extensions: &["so", "o"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Any(&[
                        &[Token::Literal(&[0x07])],
                        &[Token::Literal(&[0x0B])],
                        &[Token::Literal(&[0xCC])],
                        &[Token::Literal(&[0x08])],
                    ]),
                    Token::Any(&[&[Token::Literal(&[0x00])], &[Token::Literal(&[0x01])]]),
                    Token::Literal(&[0x64, 0x00]),
                ],
            },
        }],
    }],
    related_formats: &[],
};

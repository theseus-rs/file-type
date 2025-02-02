use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1140: FileFormat = FileFormat {
    id: 1_140,
    source_type: SourceType::Pronom,
    name: "MrSID Image Format (Multi-resolution Seamless Image Database)",
    extensions: &["sid"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x6D, 0x73, 0x69, 0x64])],
                },
            },
            ByteSequence {
                position_type: PositionType::EOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xFF, 0xD2, 0xFF, 0xA1])],
                },
            },
        ],
    }],
    related_formats: &[],
};

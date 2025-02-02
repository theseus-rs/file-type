use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1408: FileFormat = FileFormat {
    id: 1_408,
    source_type: SourceType::Pronom,
    name: "Mork",
    extensions: &["mab", "msf", "dat"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x2F, 0x2F, 0x20, 0x3C, 0x21, 0x2D, 0x2D, 0x20, 0x3C, 0x6D, 0x64, 0x62, 0x3A,
                    0x6D, 0x6F, 0x72, 0x6B, 0x3A, 0x7A, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

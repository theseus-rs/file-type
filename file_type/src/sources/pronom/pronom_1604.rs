use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1604: FileFormat = FileFormat {
    id: 1_604,
    source_type: SourceType::Pronom,
    name: "Logical File Evidence Format",
    extensions: &["l01"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4C, 0x56, 0x46, 0x09, 0x0D, 0x0A, 0xFF, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

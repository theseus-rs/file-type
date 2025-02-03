use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1306: FileFormat = FileFormat {
    id: 1_306,
    source_type: SourceType::Pronom,
    name: "Polynomial Texture Map",
    extensions: &["ptm"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x54, 0x4D, 0x5F, 0x31, 0x2E, 0x32, 0x0A, 0x50, 0x54, 0x4D, 0x5F, 0x46,
                    0x4F, 0x52, 0x4D, 0x41, 0x54, 0x5F,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

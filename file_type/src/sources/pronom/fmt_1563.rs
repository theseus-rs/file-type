use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1563: FileFormat = FileFormat {
    id: 2_388,
    puid: "fmt/1563",
    name: "ERDAS Imagine Large Raster Spill File",
    extensions: &["ige"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x45, 0x52, 0x44, 0x41, 0x53, 0x5F, 0x49, 0x4D, 0x47, 0x5F, 0x45, 0x58, 0x54,
                    0x45, 0x52, 0x4E, 0x41, 0x4C, 0x5F, 0x52, 0x41, 0x53, 0x54, 0x45, 0x52,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

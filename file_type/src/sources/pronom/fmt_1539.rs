use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1539: FileFormat = FileFormat {
    id: 2_363,
    puid: "fmt/1539",
    name: "Raster Matrix Format",
    extensions: &["rsw"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x52, 0x53, 0x57, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};

use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2363: FileFormat = FileFormat {
    id: 2_363,
    source_type: SourceType::Pronom,
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

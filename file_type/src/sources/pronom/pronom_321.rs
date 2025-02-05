use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_321: FileFormat = FileFormat {
    id: 321,
    source_type: SourceType::Pronom,
    name: "Intergraph Raster Image",
    extensions: &["ing"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

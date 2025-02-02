use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_321: FileFormat = FileFormat {
    id: 321,
    source_type: SourceType::Pronom,
    name: "Intergraph Raster Image",
    extensions: &["ing"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

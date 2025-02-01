use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_229: FileFormat = FileFormat {
    id: 321,
    puid: "x-fmt/229",
    name: "Intergraph Raster Image",
    extensions: &["ing"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

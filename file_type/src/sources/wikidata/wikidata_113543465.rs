use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113543465: FileFormat = FileFormat {
    id: 113_543_465,
    source_type: SourceType::Wikidata,
    name: "Esri Shapefile Geospatial Metadata File",
    extensions: &["xml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

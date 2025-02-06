use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113543465: FileFormat = FileFormat {
    id: 113_543_465,
    source_type: SourceType::Wikidata,
    name: "Esri Shapefile Geospatial Metadata File",
    extensions: &["xml"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

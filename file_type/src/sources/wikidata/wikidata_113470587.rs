use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113470587: FileFormat = FileFormat {
    id: 113_470_587,
    source_type: SourceType::Wikidata,
    name: "ESRI ArcGIS Raw Raster Reader/ Writer",
    extensions: &["hdr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

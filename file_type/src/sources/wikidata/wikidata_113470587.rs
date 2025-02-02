use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113470587: FileFormat = FileFormat {
    id: 113_470_587,
    source_type: SourceType::Wikidata,
    name: "ESRI ArcGIS Raw Raster Reader/ Writer",
    extensions: &["hdr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

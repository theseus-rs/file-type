use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125133522: FileFormat = FileFormat {
    id: 125_133_522,
    source_type: SourceType::Wikidata,
    name: "ArcGIS Pro Layer file",
    extensions: &["lyr", "lyrx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125133584: FileFormat = FileFormat {
    id: 125_133_584,
    source_type: SourceType::Wikidata,
    name: "ArcGIS Pro Layer package",
    extensions: &["lpkx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

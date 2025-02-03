use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125133635: FileFormat = FileFormat {
    id: 125_133_635,
    source_type: SourceType::Wikidata,
    name: "ArcGIS Pro Project Template",
    extensions: &["aptx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

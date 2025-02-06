use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125133635: FileFormat = FileFormat {
    id: 125_133_635,
    source_type: SourceType::Wikidata,
    name: "ArcGIS Pro Project Template",
    extensions: &["aptx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

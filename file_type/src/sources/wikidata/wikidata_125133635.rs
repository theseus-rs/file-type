use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125133635: FileFormat = FileFormat {
    id: 125_133_635,
    puid: "wikidata/125133635",
    name: "ArcGIS Pro Project Template",
    extensions: &["aptx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

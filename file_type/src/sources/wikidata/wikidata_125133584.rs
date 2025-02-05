use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125133584: FileFormat = FileFormat {
    id: 125_133_584,
    source_type: SourceType::Wikidata,
    name: "ArcGIS Pro Layer package",
    extensions: &["lpkx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

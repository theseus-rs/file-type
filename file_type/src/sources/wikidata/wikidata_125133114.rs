use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125133114: FileFormat = FileFormat {
    id: 125_133_114,
    source_type: SourceType::Wikidata,
    name: "ArcGIS Pro Project Package",
    extensions: &["ppkx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

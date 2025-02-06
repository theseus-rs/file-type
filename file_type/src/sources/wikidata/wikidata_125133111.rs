use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125133111: FileFormat = FileFormat {
    id: 125_133_111,
    source_type: SourceType::Wikidata,
    name: "ArcGIS Pro Project file",
    extensions: &["aprx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

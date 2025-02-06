use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125133143: FileFormat = FileFormat {
    id: 125_133_143,
    source_type: SourceType::Wikidata,
    name: "ArcGIS Pro Map file",
    extensions: &["mapx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

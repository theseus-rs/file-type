use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125133556: FileFormat = FileFormat {
    id: 125_133_556,
    source_type: SourceType::Wikidata,
    name: "ArcGIS Pro Map package",
    extensions: &["mpkx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125133233: FileFormat = FileFormat {
    id: 125_133_233,
    source_type: SourceType::Wikidata,
    name: "ArcGIS Pro Layout file",
    extensions: &["pagx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

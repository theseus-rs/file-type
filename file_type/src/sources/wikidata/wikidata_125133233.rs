use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125133233: FileFormat = FileFormat {
    id: 125_133_233,
    source_type: SourceType::Wikidata,
    name: "ArcGIS Pro Layout file",
    extensions: &["pagx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

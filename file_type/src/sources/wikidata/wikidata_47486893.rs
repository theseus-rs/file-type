use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_47486893: FileFormat = FileFormat {
    id: 47_486_893,
    source_type: SourceType::Wikidata,
    name: "Statistical Analysis System Catalog (Unix)",
    extensions: &["sas7cat", "sc7"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

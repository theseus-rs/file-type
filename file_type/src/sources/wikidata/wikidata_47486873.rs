use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_47486873: FileFormat = FileFormat {
    id: 47_486_873,
    source_type: SourceType::Wikidata,
    name: "Statistical Analysis System Catalog (Windows)",
    extensions: &["sas7bcat", "sc7"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

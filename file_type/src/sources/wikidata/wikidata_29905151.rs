use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29905151: FileFormat = FileFormat {
    id: 29_905_151,
    source_type: SourceType::Wikidata,
    name: "Statistical Analysis System permanent utility file",
    extensions: &["sas7bput", "sp2", "sp7"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

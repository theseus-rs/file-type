use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_44069213: FileFormat = FileFormat {
    id: 44_069_213,
    source_type: SourceType::Wikidata,
    name: "STATA Data File Format, version 104",
    extensions: &["dta"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};

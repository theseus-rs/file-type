use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_44071250: FileFormat = FileFormat {
    id: 44_071_250,
    source_type: SourceType::Wikidata,
    name: "STATA Data File Format, version 115",
    extensions: &["dta"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};

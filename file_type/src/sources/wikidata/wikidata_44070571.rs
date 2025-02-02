use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_44070571: FileFormat = FileFormat {
    id: 44_070_571,
    source_type: SourceType::Wikidata,
    name: "STATA Data File Format, version 114",
    extensions: &["dta"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};

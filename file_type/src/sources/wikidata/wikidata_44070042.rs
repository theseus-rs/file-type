use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_44070042: FileFormat = FileFormat {
    id: 44_070_042,
    source_type: SourceType::Wikidata,
    name: "STATA Data File Format, version 111",
    extensions: &["dta"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_32979463: FileFormat = FileFormat {
    id: 32_979_463,
    source_type: SourceType::Wikidata,
    name: "STATA DTA file format, version 119",
    extensions: &["dta"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_32979267: FileFormat = FileFormat {
    id: 32_979_267,
    source_type: SourceType::Wikidata,
    name: "STATA Data File Format, version 118",
    extensions: &["dta"],
    media_types: &["application/octet-stream"],
    signatures: &[],
    related_formats: &[],
};

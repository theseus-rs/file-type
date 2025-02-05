use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_44069766: FileFormat = FileFormat {
    id: 44_069_766,
    source_type: SourceType::Wikidata,
    name: "STATA Data File Format, version 110",
    extensions: &["dta"],
    media_types: &["application/octet-stream"],
    signatures: &[],
    related_formats: &[],
};

use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_44070148: FileFormat = FileFormat {
    id: 44_070_148,
    source_type: SourceType::Wikidata,
    name: "STATA Data File Format, version 113",
    extensions: &["dta"],
    media_types: &["application/octet-stream"],
    signatures: &[],
    related_formats: &[],
};

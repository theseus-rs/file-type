use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_49989968: FileFormat = FileFormat {
    id: 49_989_968,
    source_type: SourceType::Wikidata,
    name: "Internet Explorer for Mac cache file",
    extensions: &["waf"],
    media_types: &["application/octet-stream"],
    signatures: &[],
    related_formats: &[],
};

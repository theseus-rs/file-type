use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_49989968: FileFormat = FileFormat {
    id: 49_989_968,
    puid: "wikidata/49989968",
    name: "Internet Explorer for Mac cache file",
    extensions: &["waf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};

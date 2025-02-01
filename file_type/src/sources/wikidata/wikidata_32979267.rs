use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_32979267: FileFormat = FileFormat {
    id: 32_979_267,
    puid: "wikidata/32979267",
    name: "STATA Data File Format, version 118",
    extensions: &["dta"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};

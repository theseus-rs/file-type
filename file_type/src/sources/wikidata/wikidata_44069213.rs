use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_44069213: FileFormat = FileFormat {
    id: 44_069_213,
    puid: "wikidata/44069213",
    name: "STATA Data File Format, version 104",
    extensions: &["dta"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};

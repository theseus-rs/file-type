use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_44070571: FileFormat = FileFormat {
    id: 44_070_571,
    puid: "wikidata/44070571",
    name: "STATA Data File Format, version 114",
    extensions: &["dta"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};

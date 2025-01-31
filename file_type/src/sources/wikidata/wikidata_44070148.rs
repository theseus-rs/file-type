use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_44070148: FileFormat = FileFormat {
    id: 44_070_148,
    puid: "wikidata/44070148",
    name: "STATA Data File Format, version 113",
    extensions: &["dta"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};

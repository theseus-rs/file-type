use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_44069766: FileFormat = FileFormat {
    id: 44_069_766,
    puid: "wikidata/44069766",
    name: "STATA Data File Format, version 110",
    extensions: &["dta"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};

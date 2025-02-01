use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_44069463: FileFormat = FileFormat {
    id: 44_069_463,
    puid: "wikidata/44069463",
    name: "STATA Data File Format, version 105",
    extensions: &["dta"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};

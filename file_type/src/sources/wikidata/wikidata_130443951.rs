use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130443951: FileFormat = FileFormat {
    id: 130_443_951,
    puid: "wikidata/130443951",
    name: "Subsembly JSON",
    extensions: &["json"],
    media_types: &["application/json"],
    internal_signatures: &[],
    related_formats: &[],
};

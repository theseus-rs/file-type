use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28452000: FileFormat = FileFormat {
    id: 28_452_000,
    puid: "wikidata/28452000",
    name: "TERSE",
    extensions: &["trs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

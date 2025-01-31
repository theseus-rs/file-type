use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_21849093: FileFormat = FileFormat {
    id: 21_849_093,
    puid: "wikidata/21849093",
    name: "DIMACS standard format",
    extensions: &["col", "col.b"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

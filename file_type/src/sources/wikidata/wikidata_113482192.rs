use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113482192: FileFormat = FileFormat {
    id: 113_482_192,
    puid: "wikidata/113482192",
    name: "Calc602 Spreadsheet file or backup file v.1.00",
    extensions: &["bak", "tc6"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

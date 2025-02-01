use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113481871: FileFormat = FileFormat {
    id: 113_481_871,
    puid: "wikidata/113481871",
    name: "Calc602 Spreadsheet file or backup file",
    extensions: &["bak", "tc6"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

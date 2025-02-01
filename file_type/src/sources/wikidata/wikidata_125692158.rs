use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125692158: FileFormat = FileFormat {
    id: 125_692_158,
    puid: "wikidata/125692158",
    name: "OpenDocument Spreadsheet Template",
    extensions: &["ots"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

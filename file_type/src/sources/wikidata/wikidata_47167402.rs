use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47167402: FileFormat = FileFormat {
    id: 47_167_402,
    puid: "wikidata/47167402",
    name: "ClarisWorks Spreadsheet file format",
    extensions: &["cwk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

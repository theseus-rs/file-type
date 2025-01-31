use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47202526: FileFormat = FileFormat {
    id: 47_202_526,
    puid: "wikidata/47202526",
    name: "AppleWorks Spreadsheet file format version 6",
    extensions: &["cwk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

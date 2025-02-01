use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47195851: FileFormat = FileFormat {
    id: 47_195_851,
    puid: "wikidata/47195851",
    name: "AppleWorks Spreadsheet file format, version 5",
    extensions: &["cwk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

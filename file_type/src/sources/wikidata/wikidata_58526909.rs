use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_58526909: FileFormat = FileFormat {
    id: 58_526_909,
    puid: "wikidata/58526909",
    name: "SuperCalc Spreadsheet, version 1",
    extensions: &["cal"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

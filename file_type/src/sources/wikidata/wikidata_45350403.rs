use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_45350403: FileFormat = FileFormat {
    id: 45_350_403,
    puid: "wikidata/45350403",
    name: "Lotus 1-2-3 Spreadsheet Formatting File, version 2.3-2.4",
    extensions: &["fm1", "fmt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

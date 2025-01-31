use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_52063384: FileFormat = FileFormat {
    id: 52_063_384,
    puid: "wikidata/52063384",
    name: "SuperCalc Spreadsheet, version 4",
    extensions: &["cal"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

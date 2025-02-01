use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_52063391: FileFormat = FileFormat {
    id: 52_063_391,
    puid: "wikidata/52063391",
    name: "SuperCalc Spreadsheet, version 5",
    extensions: &["cal"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

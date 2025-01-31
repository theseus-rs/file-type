use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_62414890: FileFormat = FileFormat {
    id: 62_414_890,
    puid: "wikidata/62414890",
    name: "Quattro Pro Spreadsheet, version 7-8",
    extensions: &["wb3"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

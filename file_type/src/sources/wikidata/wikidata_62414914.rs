use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_62414914: FileFormat = FileFormat {
    id: 62_414_914,
    puid: "wikidata/62414914",
    name: "Quattro Pro Spreadsheet, version 9",
    extensions: &["qpw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

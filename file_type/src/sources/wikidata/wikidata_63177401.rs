use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_63177401: FileFormat = FileFormat {
    id: 63_177_401,
    puid: "wikidata/63177401",
    name: "Microsoft Works Spreadsheet for Macintosh, version 4",
    extensions: &["wks"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

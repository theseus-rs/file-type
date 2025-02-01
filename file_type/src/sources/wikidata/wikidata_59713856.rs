use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_59713856: FileFormat = FileFormat {
    id: 59_713_856,
    puid: "wikidata/59713856",
    name: "Microsoft Works Spreadsheet",
    extensions: &["xlr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

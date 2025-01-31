use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28771324: FileFormat = FileFormat {
    id: 28_771_324,
    puid: "wikidata/28771324",
    name: "Microsoft Works Spreadsheet",
    extensions: &["wps"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

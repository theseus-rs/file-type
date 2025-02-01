use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28804255: FileFormat = FileFormat {
    id: 28_804_255,
    puid: "wikidata/28804255",
    name: "Uniform Office Spreadsheet",
    extensions: &["uos"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_2313301: FileFormat = FileFormat {
    id: 2_313_301,
    puid: "wikidata/2313301",
    name: "SpreadsheetML",
    extensions: &["xml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

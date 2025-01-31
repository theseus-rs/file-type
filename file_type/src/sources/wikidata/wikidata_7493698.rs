use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_7493698: FileFormat = FileFormat {
    id: 7_493_698,
    puid: "wikidata/7493698",
    name: "Shell Scrap Object File",
    extensions: &["shs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_5924007: FileFormat = FileFormat {
    id: 5_924_007,
    puid: "wikidata/5924007",
    name: "JavaScript format",
    extensions: &["js"],
    media_types: &["text/javascript"],
    internal_signatures: &[],
    related_formats: &[],
};

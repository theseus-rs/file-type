use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_66711987: FileFormat = FileFormat {
    id: 66_711_987,
    puid: "wikidata/66711987",
    name: "Word Macro-Enabled Template",
    extensions: &["dotm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

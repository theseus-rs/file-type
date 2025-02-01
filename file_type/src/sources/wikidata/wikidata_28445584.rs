use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28445584: FileFormat = FileFormat {
    id: 28_445_584,
    puid: "wikidata/28445584",
    name: "Application Label Data",
    extensions: &["axc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

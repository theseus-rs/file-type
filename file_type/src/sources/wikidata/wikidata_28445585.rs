use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28445585: FileFormat = FileFormat {
    id: 28_445_585,
    puid: "wikidata/28445585",
    name: "Application Label Index",
    extensions: &["axc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

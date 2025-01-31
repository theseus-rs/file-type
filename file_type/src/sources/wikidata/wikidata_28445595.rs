use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28445595: FileFormat = FileFormat {
    id: 28_445_595,
    puid: "wikidata/28445595",
    name: "Application Object Index",
    extensions: &["axc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

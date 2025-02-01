use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_12071934: FileFormat = FileFormat {
    id: 12_071_934,
    puid: "wikidata/12071934",
    name: "nl",
    extensions: &["nl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

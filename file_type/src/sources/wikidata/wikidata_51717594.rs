use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_51717594: FileFormat = FileFormat {
    id: 51_717_594,
    puid: "wikidata/51717594",
    name: "Pocket Word Template",
    extensions: &["pwt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

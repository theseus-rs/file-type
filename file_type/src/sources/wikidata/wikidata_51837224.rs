use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_51837224: FileFormat = FileFormat {
    id: 51_837_224,
    puid: "wikidata/51837224",
    name: "Paradox Database Table, version 7",
    extensions: &["db"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

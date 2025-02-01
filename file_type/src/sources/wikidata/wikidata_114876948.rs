use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_114876948: FileFormat = FileFormat {
    id: 114_876_948,
    puid: "wikidata/114876948",
    name: "Quicken Home Inventory file",
    extensions: &["idb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_114876948: FileFormat = FileFormat {
    id: 114_876_948,
    source_type: SourceType::Wikidata,
    name: "Quicken Home Inventory file",
    extensions: &["idb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28106114: FileFormat = FileFormat {
    id: 28_106_114,
    source_type: SourceType::Wikidata,
    name: "GRASP font",
    extensions: &["fnt", "set"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28106114: FileFormat = FileFormat {
    id: 28_106_114,
    source_type: SourceType::Wikidata,
    name: "GRASP font",
    extensions: &["fnt", "set"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

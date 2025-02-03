use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_130405004: FileFormat = FileFormat {
    id: 130_405_004,
    source_type: SourceType::Wikidata,
    name: "Org file",
    extensions: &["org"],
    media_types: &["text/org"],
    internal_signatures: &[],
    related_formats: &[],
};

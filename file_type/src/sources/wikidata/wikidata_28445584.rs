use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28445584: FileFormat = FileFormat {
    id: 28_445_584,
    source_type: SourceType::Wikidata,
    name: "Application Label Data",
    extensions: &["axc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_66711987: FileFormat = FileFormat {
    id: 66_711_987,
    source_type: SourceType::Wikidata,
    name: "Word Macro-Enabled Template",
    extensions: &["dotm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

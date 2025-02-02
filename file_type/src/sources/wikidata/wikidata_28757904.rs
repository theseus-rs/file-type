use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28757904: FileFormat = FileFormat {
    id: 28_757_904,
    source_type: SourceType::Wikidata,
    name: "Go script",
    extensions: &["go"],
    media_types: &["text/x-gosrc"],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_3807693: FileFormat = FileFormat {
    id: 3_807_693,
    source_type: SourceType::Wikidata,
    name: "ASCII tab",
    extensions: &["btab", "tab", "txt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

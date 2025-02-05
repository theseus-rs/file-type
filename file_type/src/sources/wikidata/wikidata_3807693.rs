use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_3807693: FileFormat = FileFormat {
    id: 3_807_693,
    source_type: SourceType::Wikidata,
    name: "ASCII tab",
    extensions: &["btab", "tab", "txt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

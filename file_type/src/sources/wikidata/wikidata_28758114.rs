use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28758114: FileFormat = FileFormat {
    id: 28_758_114,
    source_type: SourceType::Wikidata,
    name: "Internet Shortcut",
    extensions: &["url"],
    media_types: &["application/x-mswinurl"],
    signatures: &[],
    related_formats: &[],
};

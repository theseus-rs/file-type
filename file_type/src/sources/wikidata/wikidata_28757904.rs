use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28757904: FileFormat = FileFormat {
    id: 28_757_904,
    source_type: SourceType::Wikidata,
    name: "Go script",
    extensions: &["go"],
    media_types: &["text/x-gosrc"],
    signatures: &[],
    related_formats: &[],
};

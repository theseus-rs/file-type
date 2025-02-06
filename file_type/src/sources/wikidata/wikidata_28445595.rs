use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28445595: FileFormat = FileFormat {
    id: 28_445_595,
    source_type: SourceType::Wikidata,
    name: "Application Object Index",
    extensions: &["axc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28445586: FileFormat = FileFormat {
    id: 28_445_586,
    source_type: SourceType::Wikidata,
    name: "Application Label Temporary",
    extensions: &["axc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28445585: FileFormat = FileFormat {
    id: 28_445_585,
    source_type: SourceType::Wikidata,
    name: "Application Label Index",
    extensions: &["axc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

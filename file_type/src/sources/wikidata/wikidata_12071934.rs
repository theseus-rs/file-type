use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_12071934: FileFormat = FileFormat {
    id: 12_071_934,
    source_type: SourceType::Wikidata,
    name: "nl",
    extensions: &["nl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

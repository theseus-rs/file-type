use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_12071934: FileFormat = FileFormat {
    id: 12_071_934,
    source_type: SourceType::Wikidata,
    name: "nl",
    extensions: &["nl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_108328831: FileFormat = FileFormat {
    id: 108_328_831,
    source_type: SourceType::Wikidata,
    name: "Universe Sandbox Data File",
    extensions: &["ubox"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206584: FileFormat = FileFormat {
    id: 28_206_584,
    source_type: SourceType::Wikidata,
    name: "MGR bitmap",
    extensions: &["mgr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

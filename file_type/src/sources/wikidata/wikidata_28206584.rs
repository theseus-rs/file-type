use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206584: FileFormat = FileFormat {
    id: 28_206_584,
    source_type: SourceType::Wikidata,
    name: "MGR bitmap",
    extensions: &["mgr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

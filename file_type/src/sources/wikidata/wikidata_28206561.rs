use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206561: FileFormat = FileFormat {
    id: 28_206_561,
    source_type: SourceType::Wikidata,
    name: "Maya IFF",
    extensions: &["iff", "tdi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206561: FileFormat = FileFormat {
    id: 28_206_561,
    source_type: SourceType::Wikidata,
    name: "Maya IFF",
    extensions: &["iff", "tdi"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

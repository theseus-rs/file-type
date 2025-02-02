use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_61901831: FileFormat = FileFormat {
    id: 61_901_831,
    source_type: SourceType::Wikidata,
    name: "Peak Graphical Waveform File",
    extensions: &["pk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_123369914: FileFormat = FileFormat {
    id: 123_369_914,
    source_type: SourceType::Wikidata,
    name: "High-Throughput JPEG 2000",
    extensions: &["jph"],
    media_types: &["image/jph"],
    internal_signatures: &[],
    related_formats: &[],
};

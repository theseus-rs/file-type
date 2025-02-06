use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_123369914: FileFormat = FileFormat {
    id: 123_369_914,
    source_type: SourceType::Wikidata,
    name: "High-Throughput JPEG 2000",
    extensions: &["jph"],
    media_types: &["image/jph"],
    signatures: &[],
    related_formats: &[],
};

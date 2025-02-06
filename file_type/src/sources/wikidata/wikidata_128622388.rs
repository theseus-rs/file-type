use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_128622388: FileFormat = FileFormat {
    id: 128_622_388,
    source_type: SourceType::Wikidata,
    name: "Augeas file format",
    extensions: &["aug"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

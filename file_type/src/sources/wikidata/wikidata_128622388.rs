use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_128622388: FileFormat = FileFormat {
    id: 128_622_388,
    source_type: SourceType::Wikidata,
    name: "Augeas file format",
    extensions: &["aug"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

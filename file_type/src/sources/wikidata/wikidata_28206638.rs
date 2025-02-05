use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206638: FileFormat = FileFormat {
    id: 28_206_638,
    source_type: SourceType::Wikidata,
    name: "MTV ray tracer bitmap",
    extensions: &["mtv", "pic"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

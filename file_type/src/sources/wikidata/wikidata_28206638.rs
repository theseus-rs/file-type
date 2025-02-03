use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206638: FileFormat = FileFormat {
    id: 28_206_638,
    source_type: SourceType::Wikidata,
    name: "MTV ray tracer bitmap",
    extensions: &["mtv", "pic"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

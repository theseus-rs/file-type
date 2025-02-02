use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206433: FileFormat = FileFormat {
    id: 28_206_433,
    source_type: SourceType::Wikidata,
    name: "JPEG 2000 compound image",
    extensions: &["jpm"],
    media_types: &["image/jpm"],
    internal_signatures: &[],
    related_formats: &[],
};

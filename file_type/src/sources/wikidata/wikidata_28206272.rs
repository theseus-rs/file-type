use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206272: FileFormat = FileFormat {
    id: 28_206_272,
    source_type: SourceType::Wikidata,
    name: "HTC splashscreen",
    extensions: &["img", "nb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

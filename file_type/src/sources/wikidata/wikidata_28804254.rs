use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28804254: FileFormat = FileFormat {
    id: 28_804_254,
    source_type: SourceType::Wikidata,
    name: "Uniform Office Text",
    extensions: &["uot"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

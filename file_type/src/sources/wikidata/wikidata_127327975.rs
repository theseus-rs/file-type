use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_127327975: FileFormat = FileFormat {
    id: 127_327_975,
    source_type: SourceType::Wikidata,
    name: "CUDA file",
    extensions: &["cu"],
    media_types: &["text/x-cuda"],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206373: FileFormat = FileFormat {
    id: 28_206_373,
    source_type: SourceType::Wikidata,
    name: "Interleaf image",
    extensions: &["iimg", "img"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206252: FileFormat = FileFormat {
    id: 28_206_252,
    source_type: SourceType::Wikidata,
    name: "HMR",
    extensions: &["hmr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

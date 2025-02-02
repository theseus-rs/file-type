use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206378: FileFormat = FileFormat {
    id: 28_206_378,
    source_type: SourceType::Wikidata,
    name: "IPI",
    extensions: &["ipi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

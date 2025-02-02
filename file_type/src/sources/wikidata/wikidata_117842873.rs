use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117842873: FileFormat = FileFormat {
    id: 117_842_873,
    source_type: SourceType::Wikidata,
    name: "EDMICS 6",
    extensions: &["ed6"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

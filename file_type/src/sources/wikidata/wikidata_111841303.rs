use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111841303: FileFormat = FileFormat {
    id: 111_841_303,
    source_type: SourceType::Wikidata,
    name: "line-delimited JSON",
    extensions: &["ldjson"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

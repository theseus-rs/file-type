use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111841144: FileFormat = FileFormat {
    id: 111_841_144,
    source_type: SourceType::Wikidata,
    name: "JSON Lines",
    extensions: &["jsonl"],
    media_types: &["application/jsonl"],
    internal_signatures: &[],
    related_formats: &[],
};

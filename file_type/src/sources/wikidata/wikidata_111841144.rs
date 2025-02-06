use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111841144: FileFormat = FileFormat {
    id: 111_841_144,
    source_type: SourceType::Wikidata,
    name: "JSON Lines",
    extensions: &["jsonl"],
    media_types: &["application/jsonl"],
    signatures: &[],
    related_formats: &[],
};

use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111841303: FileFormat = FileFormat {
    id: 111_841_303,
    source_type: SourceType::Wikidata,
    name: "line-delimited JSON",
    extensions: &["ldjson"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

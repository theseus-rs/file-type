use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111841242: FileFormat = FileFormat {
    id: 111_841_242,
    source_type: SourceType::Wikidata,
    name: "NDJSON",
    extensions: &["ndjson"],
    media_types: &["application/x-ndjson"],
    signatures: &[],
    related_formats: &[],
};

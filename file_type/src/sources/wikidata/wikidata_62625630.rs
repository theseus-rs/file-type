use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_62625630: FileFormat = FileFormat {
    id: 62_625_630,
    source_type: SourceType::Wikidata,
    name: "Structured Query Language script",
    extensions: &["sql"],
    media_types: &["application/sql", "text/x-sql"],
    signatures: &[],
    related_formats: &[],
};

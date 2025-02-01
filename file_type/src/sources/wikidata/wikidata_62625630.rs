use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_62625630: FileFormat = FileFormat {
    id: 62_625_630,
    puid: "wikidata/62625630",
    name: "Structured Query Language script",
    extensions: &["sql", "sql"],
    media_types: &["application/sql", "text/x-sql"],
    internal_signatures: &[],
    related_formats: &[],
};

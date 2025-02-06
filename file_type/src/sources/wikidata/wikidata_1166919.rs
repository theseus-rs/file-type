use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1166919: FileFormat = FileFormat {
    id: 1_166_919,
    source_type: SourceType::Wikidata,
    name: "Darwin Information Typing Architecture",
    extensions: &["dita", "xml"],
    media_types: &["application/dita+xml"],
    signatures: &[],
    related_formats: &[],
};

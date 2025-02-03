use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_1166919: FileFormat = FileFormat {
    id: 1_166_919,
    source_type: SourceType::Wikidata,
    name: "Darwin Information Typing Architecture",
    extensions: &["dita", "xml"],
    media_types: &["application/dita+xml"],
    internal_signatures: &[],
    related_formats: &[],
};

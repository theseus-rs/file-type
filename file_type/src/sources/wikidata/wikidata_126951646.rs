use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_126951646: FileFormat = FileFormat {
    id: 126_951_646,
    source_type: SourceType::Wikidata,
    name: "Lex source file",
    extensions: &["l"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

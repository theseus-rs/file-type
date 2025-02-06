use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_126951646: FileFormat = FileFormat {
    id: 126_951_646,
    source_type: SourceType::Wikidata,
    name: "Lex source file",
    extensions: &["l"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

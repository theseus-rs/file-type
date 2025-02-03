use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_130349380: FileFormat = FileFormat {
    id: 130_349_380,
    source_type: SourceType::Wikidata,
    name: "Modula-2 source code file",
    extensions: &["def", "mod"],
    media_types: &["text/x-modula2"],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_129082474: FileFormat = FileFormat {
    id: 129_082_474,
    source_type: SourceType::Wikidata,
    name: "Elixir script file",
    extensions: &["exs"],
    media_types: &["text/x-elixir"],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_129082474: FileFormat = FileFormat {
    id: 129_082_474,
    source_type: SourceType::Wikidata,
    name: "Elixir script file",
    extensions: &["exs"],
    media_types: &["text/x-elixir"],
    signatures: &[],
    related_formats: &[],
};

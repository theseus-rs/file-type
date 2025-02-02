use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_129081321: FileFormat = FileFormat {
    id: 129_081_321,
    source_type: SourceType::Wikidata,
    name: "Elixir source code file",
    extensions: &["ex"],
    media_types: &["text/x-elixir"],
    internal_signatures: &[],
    related_formats: &[],
};

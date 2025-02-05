use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_129081321: FileFormat = FileFormat {
    id: 129_081_321,
    source_type: SourceType::Wikidata,
    name: "Elixir source code file",
    extensions: &["ex"],
    media_types: &["text/x-elixir"],
    signatures: &[],
    related_formats: &[],
};

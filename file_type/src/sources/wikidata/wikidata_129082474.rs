use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_129082474: FileFormat = FileFormat {
    id: 129_082_474,
    puid: "wikidata/129082474",
    name: "Elixir script file",
    extensions: &["exs"],
    media_types: &["text/x-elixir"],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_126818513: FileFormat = FileFormat {
    id: 126_818_513,
    source_type: SourceType::Wikidata,
    name: "Erlang source code file",
    extensions: &["erl"],
    media_types: &["text/x-erlang"],
    internal_signatures: &[],
    related_formats: &[],
};

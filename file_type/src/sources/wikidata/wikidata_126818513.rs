use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_126818513: FileFormat = FileFormat {
    id: 126_818_513,
    puid: "wikidata/126818513",
    name: "Erlang source code file",
    extensions: &["erl"],
    media_types: &["text/x-erlang"],
    internal_signatures: &[],
    related_formats: &[],
};

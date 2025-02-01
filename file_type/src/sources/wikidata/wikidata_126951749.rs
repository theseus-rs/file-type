use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_126951749: FileFormat = FileFormat {
    id: 126_951_749,
    puid: "wikidata/126951749",
    name: "OCaml source code file",
    extensions: &["ml"],
    media_types: &["text/x-ocaml"],
    internal_signatures: &[],
    related_formats: &[],
};

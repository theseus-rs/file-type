use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_126951749: FileFormat = FileFormat {
    id: 126_951_749,
    source_type: SourceType::Wikidata,
    name: "OCaml source code file",
    extensions: &["ml"],
    media_types: &["text/x-ocaml"],
    signatures: &[],
    related_formats: &[],
};

use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111432414: FileFormat = FileFormat {
    id: 111_432_414,
    puid: "wikidata/111432414",
    name: "Lisp Program Source Code File",
    extensions: &["lsp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

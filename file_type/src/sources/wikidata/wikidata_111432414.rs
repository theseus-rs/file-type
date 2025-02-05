use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111432414: FileFormat = FileFormat {
    id: 111_432_414,
    source_type: SourceType::Wikidata,
    name: "Lisp Program Source Code File",
    extensions: &["lsp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

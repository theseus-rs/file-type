use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111721962: FileFormat = FileFormat {
    id: 111_721_962,
    source_type: SourceType::Wikidata,
    name: "Fortran include file",
    extensions: &["i90", "inc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_21849093: FileFormat = FileFormat {
    id: 21_849_093,
    source_type: SourceType::Wikidata,
    name: "DIMACS standard format",
    extensions: &["col", "col.b"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

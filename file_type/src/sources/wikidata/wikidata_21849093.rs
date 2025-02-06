use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_21849093: FileFormat = FileFormat {
    id: 21_849_093,
    source_type: SourceType::Wikidata,
    name: "DIMACS standard format",
    extensions: &["col", "col.b"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

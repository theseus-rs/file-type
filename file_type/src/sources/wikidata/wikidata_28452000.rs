use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28452000: FileFormat = FileFormat {
    id: 28_452_000,
    source_type: SourceType::Wikidata,
    name: "TERSE",
    extensions: &["trs"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

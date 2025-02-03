use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28452000: FileFormat = FileFormat {
    id: 28_452_000,
    source_type: SourceType::Wikidata,
    name: "TERSE",
    extensions: &["trs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

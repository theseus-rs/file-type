use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_130905435: FileFormat = FileFormat {
    id: 130_905_435,
    source_type: SourceType::Wikidata,
    name: "Sieve file format",
    extensions: &["sieve", "siv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_62561275: FileFormat = FileFormat {
    id: 62_561_275,
    source_type: SourceType::Wikidata,
    name: "Hangul Word Processor Document, version 5",
    extensions: &["hwp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

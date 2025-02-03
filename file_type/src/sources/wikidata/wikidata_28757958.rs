use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28757958: FileFormat = FileFormat {
    id: 28_757_958,
    source_type: SourceType::Wikidata,
    name: "Hangul Word Processor Document",
    extensions: &["hwp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

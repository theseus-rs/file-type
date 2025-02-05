use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28757958: FileFormat = FileFormat {
    id: 28_757_958,
    source_type: SourceType::Wikidata,
    name: "Hangul Word Processor Document",
    extensions: &["hwp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

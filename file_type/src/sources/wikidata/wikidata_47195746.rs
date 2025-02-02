use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_47195746: FileFormat = FileFormat {
    id: 47_195_746,
    source_type: SourceType::Wikidata,
    name: "AppleWorks Word Processor file format, version 5",
    extensions: &["cwk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

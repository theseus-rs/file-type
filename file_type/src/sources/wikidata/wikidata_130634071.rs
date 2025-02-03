use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_130634071: FileFormat = FileFormat {
    id: 130_634_071,
    source_type: SourceType::Wikidata,
    name: "RITA file format",
    extensions: &["rita"],
    media_types: &["text/rita"],
    internal_signatures: &[],
    related_formats: &[],
};

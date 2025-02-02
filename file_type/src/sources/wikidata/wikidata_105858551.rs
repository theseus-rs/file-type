use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858551: FileFormat = FileFormat {
    id: 105_858_551,
    source_type: SourceType::Wikidata,
    name: "Skyscraper simulator Building script",
    extensions: &["bld"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};

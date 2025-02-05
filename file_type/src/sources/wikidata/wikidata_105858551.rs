use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858551: FileFormat = FileFormat {
    id: 105_858_551,
    source_type: SourceType::Wikidata,
    name: "Skyscraper simulator Building script",
    extensions: &["bld"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};

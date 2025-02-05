use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_47197294: FileFormat = FileFormat {
    id: 47_197_294,
    source_type: SourceType::Wikidata,
    name: "AppleWorks Word Processor file format version 6",
    extensions: &["cwk"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

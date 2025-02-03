use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_47197074: FileFormat = FileFormat {
    id: 47_197_074,
    source_type: SourceType::Wikidata,
    name: "AppleWorks Drawing file format version 6",
    extensions: &["cwk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

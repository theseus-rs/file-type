use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856225: FileFormat = FileFormat {
    id: 105_856_225,
    source_type: SourceType::Wikidata,
    name: "Summation Document Image Information Load File",
    extensions: &["dii"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

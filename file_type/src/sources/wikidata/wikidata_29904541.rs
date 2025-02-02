use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29904541: FileFormat = FileFormat {
    id: 29_904_541,
    source_type: SourceType::Wikidata,
    name: "Statistical Analysis System audit file",
    extensions: &["sas7baud", "st7"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

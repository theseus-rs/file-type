use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_34289890: FileFormat = FileFormat {
    id: 34_289_890,
    source_type: SourceType::Wikidata,
    name: "SETI@Home data file",
    extensions: &["sah"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

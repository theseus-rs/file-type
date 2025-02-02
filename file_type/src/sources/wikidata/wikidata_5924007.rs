use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_5924007: FileFormat = FileFormat {
    id: 5_924_007,
    source_type: SourceType::Wikidata,
    name: "JavaScript format",
    extensions: &["js"],
    media_types: &["text/javascript"],
    internal_signatures: &[],
    related_formats: &[],
};

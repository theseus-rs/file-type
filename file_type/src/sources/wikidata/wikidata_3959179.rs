use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_3959179: FileFormat = FileFormat {
    id: 3_959_179,
    source_type: SourceType::Wikidata,
    name: "shar",
    extensions: &["sha", "shar"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

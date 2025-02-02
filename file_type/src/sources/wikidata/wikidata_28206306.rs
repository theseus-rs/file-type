use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206306: FileFormat = FileFormat {
    id: 28_206_306,
    source_type: SourceType::Wikidata,
    name: "Analyze IMG",
    extensions: &["img"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

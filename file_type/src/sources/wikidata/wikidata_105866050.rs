use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105866050: FileFormat = FileFormat {
    id: 105_866_050,
    source_type: SourceType::Wikidata,
    name: "Planner project",
    extensions: &["planner"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};

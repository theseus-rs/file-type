use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117276362: FileFormat = FileFormat {
    id: 117_276_362,
    source_type: SourceType::Wikidata,
    name: "Ultimate Business Planner File",
    extensions: &["bp1"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

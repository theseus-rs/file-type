use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28600496: FileFormat = FileFormat {
    id: 28_600_496,
    source_type: SourceType::Wikidata,
    name: "diff",
    extensions: &["diff", "patch"],
    media_types: &["text/x-patch"],
    internal_signatures: &[],
    related_formats: &[],
};

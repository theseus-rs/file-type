use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28600496: FileFormat = FileFormat {
    id: 28_600_496,
    source_type: SourceType::Wikidata,
    name: "diff",
    extensions: &["diff", "patch"],
    media_types: &["text/x-patch"],
    signatures: &[],
    related_formats: &[],
};

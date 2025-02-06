use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_100243790: FileFormat = FileFormat {
    id: 100_243_790,
    source_type: SourceType::Wikidata,
    name: "Student Writing Center Report",
    extensions: &["rp", "rpt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

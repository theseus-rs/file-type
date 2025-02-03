use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_100243790: FileFormat = FileFormat {
    id: 100_243_790,
    source_type: SourceType::Wikidata,
    name: "Student Writing Center Report",
    extensions: &["rp", "rpt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

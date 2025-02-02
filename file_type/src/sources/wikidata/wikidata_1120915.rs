use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_1120915: FileFormat = FileFormat {
    id: 1_120_915,
    source_type: SourceType::Wikidata,
    name: "Compact Disc Audio track",
    extensions: &["cda"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

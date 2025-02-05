use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1120915: FileFormat = FileFormat {
    id: 1_120_915,
    source_type: SourceType::Wikidata,
    name: "Compact Disc Audio track",
    extensions: &["cda"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

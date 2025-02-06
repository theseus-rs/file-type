use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110039733: FileFormat = FileFormat {
    id: 110_039_733,
    source_type: SourceType::Wikidata,
    name: "Mar Archive",
    extensions: &["mac", "mar"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

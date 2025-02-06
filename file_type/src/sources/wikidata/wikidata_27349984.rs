use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27349984: FileFormat = FileFormat {
    id: 27_349_984,
    source_type: SourceType::Wikidata,
    name: "TOPSAR Correlation Map",
    extensions: &["corgr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

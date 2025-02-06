use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_127274541: FileFormat = FileFormat {
    id: 127_274_541,
    source_type: SourceType::Wikidata,
    name: "Pro/ENGINEER Elysium Neutral File",
    extensions: &["enf_abq"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

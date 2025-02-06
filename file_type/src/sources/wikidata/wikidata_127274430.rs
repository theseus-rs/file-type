use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_127274430: FileFormat = FileFormat {
    id: 127_274_430,
    source_type: SourceType::Wikidata,
    name: "NX Elysium Neutral File",
    extensions: &["enf_abq"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

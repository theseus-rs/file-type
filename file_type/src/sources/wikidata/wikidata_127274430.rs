use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_127274430: FileFormat = FileFormat {
    id: 127_274_430,
    source_type: SourceType::Wikidata,
    name: "NX Elysium Neutral File",
    extensions: &["enf_abq"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

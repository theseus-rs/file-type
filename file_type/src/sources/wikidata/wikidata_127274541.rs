use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_127274541: FileFormat = FileFormat {
    id: 127_274_541,
    source_type: SourceType::Wikidata,
    name: "Pro/ENGINEER Elysium Neutral File",
    extensions: &["enf_abq"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

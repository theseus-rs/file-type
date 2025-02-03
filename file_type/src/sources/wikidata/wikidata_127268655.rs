use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_127268655: FileFormat = FileFormat {
    id: 127_268_655,
    source_type: SourceType::Wikidata,
    name: "CATIA V5 Elysium Neutral File",
    extensions: &["enf_abq"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

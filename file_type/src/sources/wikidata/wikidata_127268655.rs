use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_127268655: FileFormat = FileFormat {
    id: 127_268_655,
    source_type: SourceType::Wikidata,
    name: "CATIA V5 Elysium Neutral File",
    extensions: &["enf_abq"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

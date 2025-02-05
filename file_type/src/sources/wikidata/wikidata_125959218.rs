use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125959218: FileFormat = FileFormat {
    id: 125_959_218,
    source_type: SourceType::Wikidata,
    name: "Sibelius Score 2",
    extensions: &["sib"],
    media_types: &["application/x-sibelius-score"],
    signatures: &[],
    related_formats: &[],
};

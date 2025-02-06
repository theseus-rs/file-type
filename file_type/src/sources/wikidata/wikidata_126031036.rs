use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_126031036: FileFormat = FileFormat {
    id: 126_031_036,
    source_type: SourceType::Wikidata,
    name: "Sibelius Score 2023.5-2023.8",
    extensions: &["sib"],
    media_types: &["application/x-sibelius-score"],
    signatures: &[],
    related_formats: &[],
};

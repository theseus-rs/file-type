use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_126012109: FileFormat = FileFormat {
    id: 126_012_109,
    source_type: SourceType::Wikidata,
    name: "Sibelius Score 2022.12-2023.3",
    extensions: &["sib"],
    media_types: &["application/x-sibelius-score"],
    signatures: &[],
    related_formats: &[],
};

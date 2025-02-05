use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125999013: FileFormat = FileFormat {
    id: 125_999_013,
    source_type: SourceType::Wikidata,
    name: "Sibelius Score 6",
    extensions: &["sib"],
    media_types: &["application/x-sibelius-score"],
    signatures: &[],
    related_formats: &[],
};

use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125998297: FileFormat = FileFormat {
    id: 125_998_297,
    source_type: SourceType::Wikidata,
    name: "Sibelius Score 4",
    extensions: &["sib"],
    media_types: &["application/x-sibelius-score"],
    signatures: &[],
    related_formats: &[],
};

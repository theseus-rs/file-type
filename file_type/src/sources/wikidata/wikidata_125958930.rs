use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125958930: FileFormat = FileFormat {
    id: 125_958_930,
    source_type: SourceType::Wikidata,
    name: "Sibelius Score 1.2",
    extensions: &["sib"],
    media_types: &["application/x-sibelius-score"],
    signatures: &[],
    related_formats: &[],
};

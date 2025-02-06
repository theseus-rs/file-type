use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_126033191: FileFormat = FileFormat {
    id: 126_033_191,
    source_type: SourceType::Wikidata,
    name: "Sibelius Score 2024",
    extensions: &["sib"],
    media_types: &["application/x-sibelius-score"],
    signatures: &[],
    related_formats: &[],
};

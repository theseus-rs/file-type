use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_126010031: FileFormat = FileFormat {
    id: 126_010_031,
    source_type: SourceType::Wikidata,
    name: "Sibelius Score 8.6-2019.12",
    extensions: &["sib"],
    media_types: &["application/x-sibelius-score"],
    internal_signatures: &[],
    related_formats: &[],
};

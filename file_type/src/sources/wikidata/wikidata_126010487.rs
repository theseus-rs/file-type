use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_126010487: FileFormat = FileFormat {
    id: 126_010_487,
    source_type: SourceType::Wikidata,
    name: "Sibelius Score 2020.3-2022.5",
    extensions: &["sib"],
    media_types: &["application/x-sibelius-score"],
    signatures: &[],
    related_formats: &[],
};

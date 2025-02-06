use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125997892: FileFormat = FileFormat {
    id: 125_997_892,
    source_type: SourceType::Wikidata,
    name: "Sibelius Score 3",
    extensions: &["sib"],
    media_types: &["application/x-sibelius-score"],
    signatures: &[],
    related_formats: &[],
};

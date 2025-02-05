use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110239358: FileFormat = FileFormat {
    id: 110_239_358,
    source_type: SourceType::Wikidata,
    name: "CompanyMOVE ShowPlanner",
    extensions: &["sex"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

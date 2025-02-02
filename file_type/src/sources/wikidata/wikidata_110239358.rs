use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_110239358: FileFormat = FileFormat {
    id: 110_239_358,
    source_type: SourceType::Wikidata,
    name: "CompanyMOVE ShowPlanner",
    extensions: &["sex"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_6457314: FileFormat = FileFormat {
    id: 6_457_314,
    source_type: SourceType::Wikidata,
    name: "LBR",
    extensions: &["lbr", "lqr", "lyr", "lzr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

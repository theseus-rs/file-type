use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_127604847: FileFormat = FileFormat {
    id: 127_604_847,
    source_type: SourceType::Wikidata,
    name: "AMPL data file",
    extensions: &["dat"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

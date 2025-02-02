use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125348106: FileFormat = FileFormat {
    id: 125_348_106,
    source_type: SourceType::Wikidata,
    name: "Regularly Sampled Format",
    extensions: &["rsf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

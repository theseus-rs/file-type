use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125348106: FileFormat = FileFormat {
    id: 125_348_106,
    source_type: SourceType::Wikidata,
    name: "Regularly Sampled Format",
    extensions: &["rsf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

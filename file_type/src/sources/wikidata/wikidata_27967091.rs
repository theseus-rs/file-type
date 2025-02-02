use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27967091: FileFormat = FileFormat {
    id: 27_967_091,
    source_type: SourceType::Wikidata,
    name: "Funcom ISS",
    extensions: &["iss", "xarc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

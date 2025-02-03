use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_51922425: FileFormat = FileFormat {
    id: 51_922_425,
    source_type: SourceType::Wikidata,
    name: "Quicken Data File",
    extensions: &["abd", "qdf", "qel"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

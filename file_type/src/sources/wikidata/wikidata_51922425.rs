use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_51922425: FileFormat = FileFormat {
    id: 51_922_425,
    source_type: SourceType::Wikidata,
    name: "Quicken Data File",
    extensions: &["abd", "qdf", "qel"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113644918: FileFormat = FileFormat {
    id: 113_644_918,
    source_type: SourceType::Wikidata,
    name: "Intel SatisFAXtion",
    extensions: &["dcx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

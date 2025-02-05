use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113644918: FileFormat = FileFormat {
    id: 113_644_918,
    source_type: SourceType::Wikidata,
    name: "Intel SatisFAXtion",
    extensions: &["dcx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

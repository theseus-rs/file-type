use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_6753724: FileFormat = FileFormat {
    id: 6_753_724,
    source_type: SourceType::Wikidata,
    name: "MapInfo TAB format",
    extensions: &["tab"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_6753724: FileFormat = FileFormat {
    id: 6_753_724,
    source_type: SourceType::Wikidata,
    name: "MapInfo TAB format",
    extensions: &["tab"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

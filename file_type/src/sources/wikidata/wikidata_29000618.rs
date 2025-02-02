use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29000618: FileFormat = FileFormat {
    id: 29_000_618,
    source_type: SourceType::Wikidata,
    name: "Hiew Colour Markers",
    extensions: &["cmarkers"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

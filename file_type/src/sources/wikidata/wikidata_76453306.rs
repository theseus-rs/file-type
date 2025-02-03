use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_76453306: FileFormat = FileFormat {
    id: 76_453_306,
    source_type: SourceType::Wikidata,
    name: "MagicPoint presentation format",
    extensions: &["mgp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

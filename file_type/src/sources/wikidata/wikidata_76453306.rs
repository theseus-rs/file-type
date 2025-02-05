use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_76453306: FileFormat = FileFormat {
    id: 76_453_306,
    source_type: SourceType::Wikidata,
    name: "MagicPoint presentation format",
    extensions: &["mgp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

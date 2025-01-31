use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_76453306: FileFormat = FileFormat {
    id: 76_453_306,
    puid: "wikidata/76453306",
    name: "MagicPoint presentation format",
    extensions: &["mgp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

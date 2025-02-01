use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_123686089: FileFormat = FileFormat {
    id: 123_686_089,
    puid: "wikidata/123686089",
    name: "Adobe InDesign Document, version CC 2024",
    extensions: &["ind", "indd", "indt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_123685792: FileFormat = FileFormat {
    id: 123_685_792,
    puid: "wikidata/123685792",
    name: "Adobe InDesign Document, version CC 2023",
    extensions: &["ind", "indd", "indt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

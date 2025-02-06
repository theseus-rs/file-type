use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_123685792: FileFormat = FileFormat {
    id: 123_685_792,
    source_type: SourceType::Wikidata,
    name: "Adobe InDesign Document, version CC 2023",
    extensions: &["ind", "indd", "indt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

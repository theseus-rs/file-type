use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_123686089: FileFormat = FileFormat {
    id: 123_686_089,
    source_type: SourceType::Wikidata,
    name: "Adobe InDesign Document, version CC 2024",
    extensions: &["ind", "indd", "indt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

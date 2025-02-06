use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_114888949: FileFormat = FileFormat {
    id: 114_888_949,
    source_type: SourceType::Wikidata,
    name: "Scrapbook Factory Deluxe Photo Cube file",
    extensions: &["spq"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

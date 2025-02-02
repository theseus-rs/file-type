use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_114888949: FileFormat = FileFormat {
    id: 114_888_949,
    source_type: SourceType::Wikidata,
    name: "Scrapbook Factory Deluxe Photo Cube file",
    extensions: &["spq"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131545033: FileFormat = FileFormat {
    id: 131_545_033,
    source_type: SourceType::Wikidata,
    name: "Stanford Exploration Project file",
    extensions: &["h"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

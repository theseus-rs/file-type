use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_131545033: FileFormat = FileFormat {
    id: 131_545_033,
    source_type: SourceType::Wikidata,
    name: "Stanford Exploration Project file",
    extensions: &["h"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

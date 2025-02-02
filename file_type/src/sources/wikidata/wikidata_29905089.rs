use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29905089: FileFormat = FileFormat {
    id: 29_905_089,
    source_type: SourceType::Wikidata,
    name: "Statistical Analysis System consolidation database file",
    extensions: &["sas7bfdb", "sf2", "sf7"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

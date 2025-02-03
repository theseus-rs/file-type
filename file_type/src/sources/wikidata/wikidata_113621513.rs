use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113621513: FileFormat = FileFormat {
    id: 113_621_513,
    source_type: SourceType::Wikidata,
    name: "Load Runner Scenario file",
    extensions: &["lrs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29905104: FileFormat = FileFormat {
    id: 29_905_104,
    source_type: SourceType::Wikidata,
    name: "Statistical Analysis System data mining database file",
    extensions: &["s7m", "sas7bdmd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

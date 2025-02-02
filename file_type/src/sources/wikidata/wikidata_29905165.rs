use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29905165: FileFormat = FileFormat {
    id: 29_905_165,
    source_type: SourceType::Wikidata,
    name: "Statistical Analysis System backup file",
    extensions: &["sas7bbak", "sb7"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

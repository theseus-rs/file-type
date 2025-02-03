use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29905141: FileFormat = FileFormat {
    id: 29_905_141,
    source_type: SourceType::Wikidata,
    name: "Statistical Analysis System utility file",
    extensions: &["sas7butl", "su2", "su7"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

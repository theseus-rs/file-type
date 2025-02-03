use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_126823474: FileFormat = FileFormat {
    id: 126_823_474,
    source_type: SourceType::Wikidata,
    name: "Visual F# Script File",
    extensions: &["fsx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

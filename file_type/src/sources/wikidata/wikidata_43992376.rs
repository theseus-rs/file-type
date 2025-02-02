use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_43992376: FileFormat = FileFormat {
    id: 43_992_376,
    source_type: SourceType::Wikidata,
    name: "ABIF file format",
    extensions: &["abif"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

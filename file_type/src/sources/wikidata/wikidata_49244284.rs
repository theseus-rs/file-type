use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_49244284: FileFormat = FileFormat {
    id: 49_244_284,
    source_type: SourceType::Wikidata,
    name: "form*Z Project File",
    extensions: &["fmz"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_61912820: FileFormat = FileFormat {
    id: 61_912_820,
    source_type: SourceType::Wikidata,
    name: "ODM",
    extensions: &["odm"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_61887202: FileFormat = FileFormat {
    id: 61_887_202,
    source_type: SourceType::Wikidata,
    name: "EndNote Style File",
    extensions: &["ens"],
    media_types: &["application/x-endnote-style"],
    internal_signatures: &[],
    related_formats: &[],
};

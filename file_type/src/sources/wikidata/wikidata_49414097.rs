use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_49414097: FileFormat = FileFormat {
    id: 49_414_097,
    source_type: SourceType::Wikidata,
    name: "CATIA Model, version 4",
    extensions: &["mod", "model"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};

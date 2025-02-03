use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117338265: FileFormat = FileFormat {
    id: 117_338_265,
    source_type: SourceType::Wikidata,
    name: "Corel Catalog",
    extensions: &["clc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

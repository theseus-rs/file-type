use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117338265: FileFormat = FileFormat {
    id: 117_338_265,
    source_type: SourceType::Wikidata,
    name: "Corel Catalog",
    extensions: &["clc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

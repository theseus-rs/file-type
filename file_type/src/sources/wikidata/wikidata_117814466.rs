use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117814466: FileFormat = FileFormat {
    id: 117_814_466,
    source_type: SourceType::Wikidata,
    name: "AdTech perfectfax",
    extensions: &["adt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

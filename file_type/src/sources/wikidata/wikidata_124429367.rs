use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_124429367: FileFormat = FileFormat {
    id: 124_429_367,
    source_type: SourceType::Wikidata,
    name: "Pyramix Media File",
    extensions: &["pmf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

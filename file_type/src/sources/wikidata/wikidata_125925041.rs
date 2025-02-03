use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125925041: FileFormat = FileFormat {
    id: 125_925_041,
    source_type: SourceType::Wikidata,
    name: "Papyrus Document Template",
    extensions: &["pav"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

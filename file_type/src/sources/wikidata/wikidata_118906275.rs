use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_118906275: FileFormat = FileFormat {
    id: 118_906_275,
    source_type: SourceType::Wikidata,
    name: "ASP Configuration file",
    extensions: &["asa"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111190444: FileFormat = FileFormat {
    id: 111_190_444,
    source_type: SourceType::Wikidata,
    name: "Java Script Command File",
    extensions: &["jsf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

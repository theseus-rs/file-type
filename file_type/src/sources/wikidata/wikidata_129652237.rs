use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_129652237: FileFormat = FileFormat {
    id: 129_652_237,
    source_type: SourceType::Wikidata,
    name: "Igor Pro procedure file",
    extensions: &["ipf"],
    media_types: &["text/ipf"],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_118146053: FileFormat = FileFormat {
    id: 118_146_053,
    source_type: SourceType::Wikidata,
    name: "Microstrip File",
    extensions: &["tl1"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

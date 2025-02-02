use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_118146513: FileFormat = FileFormat {
    id: 118_146_513,
    source_type: SourceType::Wikidata,
    name: "Coaxial Cable File",
    extensions: &["tl7"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

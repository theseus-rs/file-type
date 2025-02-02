use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_114130153: FileFormat = FileFormat {
    id: 114_130_153,
    source_type: SourceType::Wikidata,
    name: "Camtasia Producer Project",
    extensions: &["cam"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

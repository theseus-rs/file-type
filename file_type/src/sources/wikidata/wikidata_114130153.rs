use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_114130153: FileFormat = FileFormat {
    id: 114_130_153,
    source_type: SourceType::Wikidata,
    name: "Camtasia Producer Project",
    extensions: &["cam"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

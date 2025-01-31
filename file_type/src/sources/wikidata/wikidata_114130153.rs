use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_114130153: FileFormat = FileFormat {
    id: 114_130_153,
    puid: "wikidata/114130153",
    name: "Camtasia Producer Project",
    extensions: &["cam"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

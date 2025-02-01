use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_115241368: FileFormat = FileFormat {
    id: 115_241_368,
    puid: "wikidata/115241368",
    name: "3D Builder Project",
    extensions: &["b3d"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

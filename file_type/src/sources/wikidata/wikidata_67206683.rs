use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_67206683: FileFormat = FileFormat {
    id: 67_206_683,
    puid: "wikidata/67206683",
    name: "VRML Worlds",
    extensions: &["3dv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

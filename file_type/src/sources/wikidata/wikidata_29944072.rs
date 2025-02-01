use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29944072: FileFormat = FileFormat {
    id: 29_944_072,
    puid: "wikidata/29944072",
    name: "Simple Voxels",
    extensions: &["svx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

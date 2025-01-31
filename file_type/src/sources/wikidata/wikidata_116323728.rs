use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_116323728: FileFormat = FileFormat {
    id: 116_323_728,
    puid: "wikidata/116323728",
    name: "Photosuite Album File",
    extensions: &["pza"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

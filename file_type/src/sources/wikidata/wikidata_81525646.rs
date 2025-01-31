use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_81525646: FileFormat = FileFormat {
    id: 81_525_646,
    puid: "wikidata/81525646",
    name: "CorelDream 3D drawing",
    extensions: &["d3d"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

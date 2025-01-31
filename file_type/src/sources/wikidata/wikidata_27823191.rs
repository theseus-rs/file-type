use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27823191: FileFormat = FileFormat {
    id: 27_823_191,
    puid: "wikidata/27823191",
    name: "Binary Terrain, version 1.0",
    extensions: &["bt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

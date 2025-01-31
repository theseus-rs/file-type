use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27823201: FileFormat = FileFormat {
    id: 27_823_201,
    puid: "wikidata/27823201",
    name: "Binary Terrain, version 1.3",
    extensions: &["bt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

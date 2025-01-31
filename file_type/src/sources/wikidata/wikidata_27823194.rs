use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27823194: FileFormat = FileFormat {
    id: 27_823_194,
    puid: "wikidata/27823194",
    name: "Binary Terrain, version 1.2",
    extensions: &["bt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

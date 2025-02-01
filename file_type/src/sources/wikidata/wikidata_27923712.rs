use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27923712: FileFormat = FileFormat {
    id: 27_923_712,
    puid: "wikidata/27923712",
    name: "DTED Level 0 Terrain Elevation Data File",
    extensions: &["dt0"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

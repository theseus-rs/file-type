use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27923715: FileFormat = FileFormat {
    id: 27_923_715,
    puid: "wikidata/27923715",
    name: "DTED Level 2 Terrain Elevation Data File",
    extensions: &["dt2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

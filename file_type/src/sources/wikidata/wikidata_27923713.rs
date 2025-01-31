use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27923713: FileFormat = FileFormat {
    id: 27_923_713,
    puid: "wikidata/27923713",
    name: "DTED Level 1 Terrain Elevation Data File",
    extensions: &["dt1"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

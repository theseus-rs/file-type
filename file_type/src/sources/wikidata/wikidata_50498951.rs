use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_50498951: FileFormat = FileFormat {
    id: 50_498_951,
    puid: "wikidata/50498951",
    name: "OGR GFS File",
    extensions: &["gfs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

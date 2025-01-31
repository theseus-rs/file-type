use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_112820809: FileFormat = FileFormat {
    id: 112_820_809,
    puid: "wikidata/112820809",
    name: "LightWave binary object file",
    extensions: &["lw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_72959401: FileFormat = FileFormat {
    id: 72_959_401,
    puid: "wikidata/72959401",
    name: "Panorama database",
    extensions: &["pan"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_112821378: FileFormat = FileFormat {
    id: 112_821_378,
    puid: "wikidata/112821378",
    name: "Minolta 3D Scanner Camera File",
    extensions: &["cam"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

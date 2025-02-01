use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_72725061: FileFormat = FileFormat {
    id: 72_725_061,
    puid: "wikidata/72725061",
    name: "NATO Secondary Imagery Format",
    extensions: &["nsf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

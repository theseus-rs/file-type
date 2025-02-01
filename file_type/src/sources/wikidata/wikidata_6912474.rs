use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_6912474: FileFormat = FileFormat {
    id: 6_912_474,
    puid: "wikidata/6912474",
    name: "Mork",
    extensions: &["dat", "mab", "msf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

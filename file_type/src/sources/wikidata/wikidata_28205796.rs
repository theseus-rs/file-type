use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205796: FileFormat = FileFormat {
    id: 28_205_796,
    puid: "wikidata/28205796",
    name: "Master of Orion saved game",
    extensions: &["gam"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

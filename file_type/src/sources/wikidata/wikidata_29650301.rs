use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29650301: FileFormat = FileFormat {
    id: 29_650_301,
    puid: "wikidata/29650301",
    name: "Pack",
    extensions: &["z"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28207424: FileFormat = FileFormat {
    id: 28_207_424,
    puid: "wikidata/28207424",
    name: "VEGX",
    extensions: &["egx", "vgx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

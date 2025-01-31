use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_11802013: FileFormat = FileFormat {
    id: 11_802_013,
    puid: "wikidata/11802013",
    name: "Sega Dreamcast Texture Package Format",
    extensions: &["pvm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

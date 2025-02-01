use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28777718: FileFormat = FileFormat {
    id: 28_777_718,
    puid: "wikidata/28777718",
    name: "National Transmission Format",
    extensions: &["ntf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

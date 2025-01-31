use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_116949770: FileFormat = FileFormat {
    id: 116_949_770,
    puid: "wikidata/116949770",
    name: "Winfax File",
    extensions: &["fxs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

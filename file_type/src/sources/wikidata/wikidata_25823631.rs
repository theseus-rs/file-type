use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_25823631: FileFormat = FileFormat {
    id: 25_823_631,
    puid: "wikidata/25823631",
    name: "MapCSS",
    extensions: &["mapcss"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_17042366: FileFormat = FileFormat {
    id: 17_042_366,
    puid: "wikidata/17042366",
    name: "id Software Music Format",
    extensions: &["imf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_61886938: FileFormat = FileFormat {
    id: 61_886_938,
    puid: "wikidata/61886938",
    name: "Portable Form File",
    extensions: &["pff"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_121786280: FileFormat = FileFormat {
    id: 121_786_280,
    puid: "wikidata/121786280",
    name: "Adobe Illustrator CC 2020 Artwork 24.2+",
    extensions: &["ai", "ait"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

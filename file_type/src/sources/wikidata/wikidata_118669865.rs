use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_118669865: FileFormat = FileFormat {
    id: 118_669_865,
    puid: "wikidata/118669865",
    name: "Manga Studio 3D Dialog file",
    extensions: &["csd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27979513: FileFormat = FileFormat {
    id: 27_979_513,
    puid: "wikidata/27979513",
    name: "Manga Studio Story",
    extensions: &["cst"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

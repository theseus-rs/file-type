use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_118672139: FileFormat = FileFormat {
    id: 118_672_139,
    puid: "wikidata/118672139",
    name: "Manga Studio 1.0 Document",
    extensions: &["mpf", "msf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

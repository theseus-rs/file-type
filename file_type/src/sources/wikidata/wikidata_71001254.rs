use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_71001254: FileFormat = FileFormat {
    id: 71_001_254,
    puid: "wikidata/71001254",
    name: "Gameboy Sound Format",
    extensions: &["gsf", "gsflib", "minigsf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

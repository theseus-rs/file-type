use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111283602: FileFormat = FileFormat {
    id: 111_283_602,
    puid: "wikidata/111283602",
    name: "Casio FZ-1 full dump format",
    extensions: &["fzf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

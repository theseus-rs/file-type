use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113487211: FileFormat = FileFormat {
    id: 113_487_211,
    puid: "wikidata/113487211",
    name: "Persuasion Windows Document 3 - 4",
    extensions: &["at3", "at4", "pn4", "pr3"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111363745: FileFormat = FileFormat {
    id: 111_363_745,
    puid: "wikidata/111363745",
    name: "Miles Sound System extended MIDI file",
    extensions: &["xmi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

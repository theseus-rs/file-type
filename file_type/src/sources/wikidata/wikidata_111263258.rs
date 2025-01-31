use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111263258: FileFormat = FileFormat {
    id: 111_263_258,
    puid: "wikidata/111263258",
    name: "Soundcap/SoundEdit instrument",
    extensions: &["dewf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

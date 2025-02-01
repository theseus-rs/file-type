use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105864369: FileFormat = FileFormat {
    id: 105_864_369,
    puid: "wikidata/105864369",
    name: "D-Fend Reloaded Profile",
    extensions: &["prof"],
    media_types: &["text/ini"],
    internal_signatures: &[],
    related_formats: &[],
};

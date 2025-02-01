use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_118669802: FileFormat = FileFormat {
    id: 118_669_802,
    puid: "wikidata/118669802",
    name: "Shade To Manga Studio file",
    extensions: &["stc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

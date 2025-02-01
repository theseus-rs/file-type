use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28345059: FileFormat = FileFormat {
    id: 28_345_059,
    puid: "wikidata/28345059",
    name: "XP3",
    extensions: &["xp3"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

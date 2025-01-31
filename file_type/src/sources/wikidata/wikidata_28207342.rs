use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28207342: FileFormat = FileFormat {
    id: 28_207_342,
    puid: "wikidata/28207342",
    name: "Synu",
    extensions: &["syn", "synu"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

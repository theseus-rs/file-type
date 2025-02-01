use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_122074126: FileFormat = FileFormat {
    id: 122_074_126,
    puid: "wikidata/122074126",
    name: "Score file",
    extensions: &["pge"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28758107: FileFormat = FileFormat {
    id: 28_758_107,
    puid: "wikidata/28758107",
    name: "Instant Artist GFX",
    extensions: &["gfx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

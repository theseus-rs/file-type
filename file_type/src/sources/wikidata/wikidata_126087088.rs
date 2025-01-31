use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_126087088: FileFormat = FileFormat {
    id: 126_087_088,
    puid: "wikidata/126087088",
    name: "IMF Package Composition Playlist",
    extensions: &["xml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

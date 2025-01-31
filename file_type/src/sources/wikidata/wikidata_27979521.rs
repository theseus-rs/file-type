use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27979521: FileFormat = FileFormat {
    id: 27_979_521,
    puid: "wikidata/27979521",
    name: "Plex Video Preview Thumbnail",
    extensions: &["bif"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

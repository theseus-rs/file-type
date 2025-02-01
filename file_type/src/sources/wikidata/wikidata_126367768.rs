use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_126367768: FileFormat = FileFormat {
    id: 126_367_768,
    puid: "wikidata/126367768",
    name: "KPhotoAlbum file",
    extensions: &["kim"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206936: FileFormat = FileFormat {
    id: 28_206_936,
    puid: "wikidata/28206936",
    name: "Photo CD",
    extensions: &["pcd", "pcd"],
    media_types: &["image/x-photo-cd", "image/x-photo-cd"],
    internal_signatures: &[],
    related_formats: &[],
};

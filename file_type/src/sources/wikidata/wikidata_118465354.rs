use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_118465354: FileFormat = FileFormat {
    id: 118_465_354,
    puid: "wikidata/118465354",
    name: "Capture One Session File",
    extensions: &["cos"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

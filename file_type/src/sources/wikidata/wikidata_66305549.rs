use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_66305549: FileFormat = FileFormat {
    id: 66_305_549,
    puid: "wikidata/66305549",
    name: "Splitted AVI File",
    extensions: &["nvavi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_66663160: FileFormat = FileFormat {
    id: 66_663_160,
    puid: "wikidata/66663160",
    name: "eSuite word processor format",
    extensions: &["html-wp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

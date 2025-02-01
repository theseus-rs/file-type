use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855536: FileFormat = FileFormat {
    id: 105_855_536,
    puid: "wikidata/105855536",
    name: "OpenColorIO profile (with rem)",
    extensions: &["ocio"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};

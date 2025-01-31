use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855578: FileFormat = FileFormat {
    id: 105_855_578,
    puid: "wikidata/105855578",
    name: "OpenShot effect",
    extensions: &["xml"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};

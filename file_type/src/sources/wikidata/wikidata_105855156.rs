use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855156: FileFormat = FileFormat {
    id: 105_855_156,
    puid: "wikidata/105855156",
    name: "Final Draft Script",
    extensions: &["fdx"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};

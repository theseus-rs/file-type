use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855797: FileFormat = FileFormat {
    id: 105_855_797,
    puid: "wikidata/105855797",
    name: "Vivid DiffSet",
    extensions: &["dsx"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};

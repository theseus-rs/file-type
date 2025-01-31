use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857854: FileFormat = FileFormat {
    id: 105_857_854,
    puid: "wikidata/105857854",
    name: "Ivy module descriptor",
    extensions: &["ivy"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

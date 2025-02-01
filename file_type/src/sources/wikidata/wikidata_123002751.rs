use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_123002751: FileFormat = FileFormat {
    id: 123_002_751,
    puid: "wikidata/123002751",
    name: "Scalable Vector Graphics 1.0",
    extensions: &["svg"],
    media_types: &["image/svg+xml"],
    internal_signatures: &[],
    related_formats: &[],
};

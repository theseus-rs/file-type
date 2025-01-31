use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_7060634: FileFormat = FileFormat {
    id: 7_060_634,
    puid: "wikidata/7060634",
    name: "Norton Guides",
    extensions: &["ng"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

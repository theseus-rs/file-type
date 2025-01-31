use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_123668205: FileFormat = FileFormat {
    id: 123_668_205,
    puid: "wikidata/123668205",
    name: "LiveCode Stack v5.5",
    extensions: &["livecode", "rev"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

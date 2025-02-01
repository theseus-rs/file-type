use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_123668527: FileFormat = FileFormat {
    id: 123_668_527,
    puid: "wikidata/123668527",
    name: "LiveCode Stack 8.1+",
    extensions: &["livecode", "rev"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

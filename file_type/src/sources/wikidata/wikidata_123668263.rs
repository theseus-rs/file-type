use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_123668263: FileFormat = FileFormat {
    id: 123_668_263,
    puid: "wikidata/123668263",
    name: "LiveCode Stack 7.0",
    extensions: &["livecode", "rev"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

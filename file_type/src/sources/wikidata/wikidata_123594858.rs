use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_123594858: FileFormat = FileFormat {
    id: 123_594_858,
    puid: "wikidata/123594858",
    name: "Portable Document Format/Archive, version 4e",
    extensions: &["pdf"],
    media_types: &["application/pdf"],
    internal_signatures: &[],
    related_formats: &[],
};

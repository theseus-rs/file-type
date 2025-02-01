use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_123595865: FileFormat = FileFormat {
    id: 123_595_865,
    puid: "wikidata/123595865",
    name: "Portable Document Format/Archive, version 4f",
    extensions: &["pdf"],
    media_types: &["application/pdf"],
    internal_signatures: &[],
    related_formats: &[],
};

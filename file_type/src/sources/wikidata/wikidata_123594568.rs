use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_123594568: FileFormat = FileFormat {
    id: 123_594_568,
    puid: "wikidata/123594568",
    name: "Portable Document Format/Archive, version 4",
    extensions: &["pdf"],
    media_types: &["application/pdf"],
    internal_signatures: &[],
    related_formats: &[],
};

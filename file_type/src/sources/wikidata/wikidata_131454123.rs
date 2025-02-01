use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131454123: FileFormat = FileFormat {
    id: 131_454_123,
    puid: "wikidata/131454123",
    name: "Zig file format",
    extensions: &["zig"],
    media_types: &["text/zig"],
    internal_signatures: &[],
    related_formats: &[],
};

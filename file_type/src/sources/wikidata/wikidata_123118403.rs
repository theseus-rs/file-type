use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_123118403: FileFormat = FileFormat {
    id: 123_118_403,
    puid: "wikidata/123118403",
    name: "PostScript 2.1",
    extensions: &["ps"],
    media_types: &["application/postscript"],
    internal_signatures: &[],
    related_formats: &[],
};

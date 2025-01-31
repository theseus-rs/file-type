use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_123118382: FileFormat = FileFormat {
    id: 123_118_382,
    puid: "wikidata/123118382",
    name: "PostScript 1.0",
    extensions: &["ps"],
    media_types: &["application/postscript"],
    internal_signatures: &[],
    related_formats: &[],
};

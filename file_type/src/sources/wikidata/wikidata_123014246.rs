use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_123014246: FileFormat = FileFormat {
    id: 123_014_246,
    puid: "wikidata/123014246",
    name: "PostScript 2.0",
    extensions: &["ps"],
    media_types: &["application/postscript"],
    internal_signatures: &[],
    related_formats: &[],
};

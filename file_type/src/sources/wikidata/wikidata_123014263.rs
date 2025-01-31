use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_123014263: FileFormat = FileFormat {
    id: 123_014_263,
    puid: "wikidata/123014263",
    name: "PostScript 3.0",
    extensions: &["ps"],
    media_types: &["application/postscript"],
    internal_signatures: &[],
    related_formats: &[],
};

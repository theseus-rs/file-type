use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_124844257: FileFormat = FileFormat {
    id: 124_844_257,
    puid: "wikidata/124844257",
    name: "MediaShow Slideshow Project File",
    extensions: &["mse"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

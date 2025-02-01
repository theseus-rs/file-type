use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_25339304: FileFormat = FileFormat {
    id: 25_339_304,
    puid: "wikidata/25339304",
    name: "Timed Text Markup Language",
    extensions: &["dxfp", "ttml", "xml"],
    media_types: &[
        "application/ttml+xml",
        "application/ttml+xml",
        "application/ttml+xml",
    ],
    internal_signatures: &[],
    related_formats: &[],
};

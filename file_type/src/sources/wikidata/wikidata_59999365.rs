use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_59999365: FileFormat = FileFormat {
    id: 59_999_365,
    puid: "wikidata/59999365",
    name: "Secure DjVU",
    extensions: &["djv", "djv", "djvu", "djvu"],
    media_types: &[
        "image/vnd.djvu",
        "image/vnd.djvu",
        "image/x-djvu",
        "image/x-djvu",
    ],
    internal_signatures: &[],
    related_formats: &[],
};

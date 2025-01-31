use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_288405: FileFormat = FileFormat {
    id: 288_405,
    puid: "wikidata/288405",
    name: "hOCR",
    extensions: &["hocr", "hocr", "html", "html"],
    media_types: &[
        "text/html",
        "text/html",
        "text/vnd.hocr+html",
        "text/vnd.hocr+html",
    ],
    internal_signatures: &[],
    related_formats: &[],
};

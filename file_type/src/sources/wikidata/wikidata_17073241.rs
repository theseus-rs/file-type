use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_17073241: FileFormat = FileFormat {
    id: 17_073_241,
    puid: "wikidata/17073241",
    name: "Opera Show Format",
    extensions: &["html", "html", "xhtml", "xhtml"],
    media_types: &[
        "application/xhtml+xml",
        "application/xhtml+xml",
        "text/html",
        "text/html",
    ],
    internal_signatures: &[],
    related_formats: &[],
};

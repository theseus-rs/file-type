use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_673906: FileFormat = FileFormat {
    id: 673_906,
    puid: "wikidata/673906",
    name: "Simple Standards-Based Slide Show System",
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

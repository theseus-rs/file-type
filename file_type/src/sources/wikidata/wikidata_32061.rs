use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_32061: FileFormat = FileFormat {
    id: 32_061,
    puid: "wikidata/32061",
    name: "XSL",
    extensions: &["xsl", "xslt"],
    media_types: &["application/xslt+xml", "application/xslt+xml"],
    internal_signatures: &[],
    related_formats: &[],
};

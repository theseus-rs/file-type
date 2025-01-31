use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131430822: FileFormat = FileFormat {
    id: 131_430_822,
    puid: "wikidata/131430822",
    name: "XQuery Source File",
    extensions: &[
        "xq", "xq", "xql", "xql", "xqm", "xqm", "xquery", "xquery", "xqy", "xqy",
    ],
    media_types: &[
        "application/xquery",
        "application/xquery",
        "application/xquery",
        "application/xquery",
        "application/xquery",
        "text/xquery",
        "text/xquery",
        "text/xquery",
        "text/xquery",
        "text/xquery",
    ],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_6059108: FileFormat = FileFormat {
    id: 6_059_108,
    puid: "wikidata/6059108",
    name: "Intuit Interchange Format",
    extensions: &["iif", "iif", "iif"],
    media_types: &["application/qbooks", "application/qbookspro", "text/iif"],
    internal_signatures: &[],
    related_formats: &[],
};

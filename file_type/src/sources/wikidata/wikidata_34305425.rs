use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_34305425: FileFormat = FileFormat {
    id: 34_305_425,
    puid: "wikidata/34305425",
    name: "Scheme script",
    extensions: &["sch", "sch", "scm", "scm", "ss", "ss"],
    media_types: &[
        "application/x-scheme",
        "application/x-scheme",
        "application/x-scheme",
        "text/x-scheme",
        "text/x-scheme",
        "text/x-scheme",
    ],
    internal_signatures: &[],
    related_formats: &[],
};

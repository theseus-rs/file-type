use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29904535: FileFormat = FileFormat {
    id: 29_904_535,
    puid: "wikidata/29904535",
    name: "Statistical Analysis System program",
    extensions: &["sas", "sas", "sas", "sas"],
    media_types: &[
        "application/octet-stream",
        "application/x-sas",
        "text/sas",
        "text/x-sas",
    ],
    internal_signatures: &[],
    related_formats: &[],
};

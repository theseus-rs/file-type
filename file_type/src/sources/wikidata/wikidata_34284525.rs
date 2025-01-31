use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_34284525: FileFormat = FileFormat {
    id: 34_284_525,
    puid: "wikidata/34284525",
    name: "Perl script",
    extensions: &["pl", "pl", "pl"],
    media_types: &[
        "application/x-httpd-perl",
        "application/x-perl",
        "text/x-perl",
    ],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_86245021: FileFormat = FileFormat {
    id: 86_245_021,
    puid: "wikidata/86245021",
    name: "BDOC 2",
    extensions: &["asice", "bdoc"],
    media_types: &[
        "application/vnd.etsi.asic-e+zip",
        "application/vnd.etsi.asic-e+zip",
    ],
    internal_signatures: &[],
    related_formats: &[],
};

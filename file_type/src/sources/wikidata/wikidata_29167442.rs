use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29167442: FileFormat = FileFormat {
    id: 29_167_442,
    puid: "wikidata/29167442",
    name: "OFIP",
    extensions: &["ofip"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

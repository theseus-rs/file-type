use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117187116: FileFormat = FileFormat {
    id: 117_187_116,
    puid: "wikidata/117187116",
    name: "CD Stomper Template file",
    extensions: &["dsu"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_51922425: FileFormat = FileFormat {
    id: 51_922_425,
    puid: "wikidata/51922425",
    name: "Quicken Data File",
    extensions: &["abd", "qdf", "qel"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113621586: FileFormat = FileFormat {
    id: 113_621_586,
    puid: "wikidata/113621586",
    name: "LoadRunner Raw Results",
    extensions: &["lrr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

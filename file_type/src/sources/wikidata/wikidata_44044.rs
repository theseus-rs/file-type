use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_44044: FileFormat = FileFormat {
    id: 44_044,
    puid: "wikidata/44044",
    name: "N-Triples",
    extensions: &["nt"],
    media_types: &["application/n-triples"],
    internal_signatures: &[],
    related_formats: &[],
};

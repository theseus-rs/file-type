use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_79242714: FileFormat = FileFormat {
    id: 79_242_714,
    puid: "wikidata/79242714",
    name: "Lotus Approach Database index",
    extensions: &["adx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

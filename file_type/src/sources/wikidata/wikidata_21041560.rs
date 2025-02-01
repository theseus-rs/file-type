use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_21041560: FileFormat = FileFormat {
    id: 21_041_560,
    puid: "wikidata/21041560",
    name: "Oktalyzer format",
    extensions: &["okt", "okta"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27959896: FileFormat = FileFormat {
    id: 27_959_896,
    puid: "wikidata/27959896",
    name: "Nuendo arrangement",
    extensions: &["npr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

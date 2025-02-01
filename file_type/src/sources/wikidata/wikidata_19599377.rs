use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_19599377: FileFormat = FileFormat {
    id: 19_599_377,
    puid: "wikidata/19599377",
    name: "AppleLink Package Compression Format",
    extensions: &["pkg"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

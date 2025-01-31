use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_33514773: FileFormat = FileFormat {
    id: 33_514_773,
    puid: "wikidata/33514773",
    name: "LAS 1.0",
    extensions: &["las", "laz"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

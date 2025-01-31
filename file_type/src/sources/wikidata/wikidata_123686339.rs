use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_123686339: FileFormat = FileFormat {
    id: 123_686_339,
    puid: "wikidata/123686339",
    name: "Digital Negative, version 1.7",
    extensions: &["dng"],
    media_types: &["image/dng"],
    internal_signatures: &[],
    related_formats: &[],
};

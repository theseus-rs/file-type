use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_7670377: FileFormat = FileFormat {
    id: 7_670_377,
    puid: "wikidata/7670377",
    name: "Tagged Image File Format/Electronic Photography",
    extensions: &["tif", "tiff"],
    media_types: &["image/tiff", "image/tiff"],
    internal_signatures: &[],
    related_formats: &[],
};

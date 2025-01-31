use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117600268: FileFormat = FileFormat {
    id: 117_600_268,
    puid: "wikidata/117600268",
    name: "Digital Negative, version 1.6",
    extensions: &["dng", "tif"],
    media_types: &["image/tiff", "image/tiff"],
    internal_signatures: &[],
    related_formats: &[],
};

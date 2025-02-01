use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117600048: FileFormat = FileFormat {
    id: 117_600_048,
    puid: "wikidata/117600048",
    name: "Digital Negative, version 1.5",
    extensions: &["dng"],
    media_types: &["image/tiff"],
    internal_signatures: &[],
    related_formats: &[],
};

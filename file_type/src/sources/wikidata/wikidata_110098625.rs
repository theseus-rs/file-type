use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110098625: FileFormat = FileFormat {
    id: 110_098_625,
    puid: "wikidata/110098625",
    name: "Exif Image File Format (Compressed)",
    extensions: &["jpeg", "jpg"],
    media_types: &["image/jpeg", "image/jpeg"],
    internal_signatures: &[],
    related_formats: &[],
};

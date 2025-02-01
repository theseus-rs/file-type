use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28207384: FileFormat = FileFormat {
    id: 28_207_384,
    puid: "wikidata/28207384",
    name: "TIFF/IT",
    extensions: &["tif", "tiff"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

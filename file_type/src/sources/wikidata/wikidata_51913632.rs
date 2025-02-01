use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_51913632: FileFormat = FileFormat {
    id: 51_913_632,
    puid: "wikidata/51913632",
    name: "SDSC Image Tool Run-Length Encoded Bitmap",
    extensions: &["rle"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

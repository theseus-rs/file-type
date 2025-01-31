use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206788: FileFormat = FileFormat {
    id: 28_206_788,
    puid: "wikidata/28206788",
    name: "OS/2 Bitmap, version 2.0",
    extensions: &["bmp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

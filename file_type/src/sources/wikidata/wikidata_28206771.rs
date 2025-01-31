use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206771: FileFormat = FileFormat {
    id: 28_206_771,
    puid: "wikidata/28206771",
    name: "OS/2 Bitmap Array",
    extensions: &["bga", "bmp", "ico"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

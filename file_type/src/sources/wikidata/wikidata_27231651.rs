use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27231651: FileFormat = FileFormat {
    id: 27_231_651,
    puid: "wikidata/27231651",
    name: "Tag Image File Format, version 5.0",
    extensions: &["tif"],
    media_types: &["image/tiff"],
    internal_signatures: &[],
    related_formats: &[],
};

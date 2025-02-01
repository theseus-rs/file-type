use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27231654: FileFormat = FileFormat {
    id: 27_231_654,
    puid: "wikidata/27231654",
    name: "Tag Image File Format, version 6.0",
    extensions: &["tif"],
    media_types: &["image/tiff"],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27231634: FileFormat = FileFormat {
    id: 27_231_634,
    puid: "wikidata/27231634",
    name: "Tag Image File Format, version 4.0",
    extensions: &["tif"],
    media_types: &["image/tiff"],
    internal_signatures: &[],
    related_formats: &[],
};

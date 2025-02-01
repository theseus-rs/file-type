use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28770329: FileFormat = FileFormat {
    id: 28_770_329,
    puid: "wikidata/28770329",
    name: "Lightwave 3D Object",
    extensions: &["lwo", "lwo"],
    media_types: &["application/x-lightwave", "image/x-lwo"],
    internal_signatures: &[],
    related_formats: &[],
};

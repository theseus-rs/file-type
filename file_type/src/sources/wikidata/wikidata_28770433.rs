use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28770433: FileFormat = FileFormat {
    id: 28_770_433,
    puid: "wikidata/28770433",
    name: "MARCXML",
    extensions: &["mrcx", "mrcx"],
    media_types: &["application/marc", "application/marcxml+xml"],
    internal_signatures: &[],
    related_formats: &[],
};

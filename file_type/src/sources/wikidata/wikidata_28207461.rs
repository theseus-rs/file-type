use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28207461: FileFormat = FileFormat {
    id: 28_207_461,
    puid: "wikidata/28207461",
    name: "VITec Image Format",
    extensions: &["vit", "vitec"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

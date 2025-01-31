use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967138: FileFormat = FileFormat {
    id: 27_967_138,
    puid: "wikidata/27967138",
    name: "DigiBooster v1.x module",
    extensions: &["digi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

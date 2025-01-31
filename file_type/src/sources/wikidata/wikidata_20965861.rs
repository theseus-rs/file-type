use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_20965861: FileFormat = FileFormat {
    id: 20_965_861,
    puid: "wikidata/20965861",
    name: "Material Template Library",
    extensions: &["mtl", "mtl"],
    media_types: &["model/mtl", "text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};

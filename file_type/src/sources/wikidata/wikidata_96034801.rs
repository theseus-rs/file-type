use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_96034801: FileFormat = FileFormat {
    id: 96_034_801,
    puid: "wikidata/96034801",
    name: "GXL",
    extensions: &["gxl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

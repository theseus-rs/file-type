use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125692808: FileFormat = FileFormat {
    id: 125_692_808,
    puid: "wikidata/125692808",
    name: "Pocket Excel Format",
    extensions: &["pxl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

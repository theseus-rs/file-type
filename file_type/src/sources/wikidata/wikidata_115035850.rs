use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_115035850: FileFormat = FileFormat {
    id: 115_035_850,
    puid: "wikidata/115035850",
    name: "Calc602 Project File 1.51",
    extensions: &["pc6"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

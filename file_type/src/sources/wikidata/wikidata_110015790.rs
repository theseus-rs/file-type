use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110015790: FileFormat = FileFormat {
    id: 110_015_790,
    puid: "wikidata/110015790",
    name: "OrCAD Layout File",
    extensions: &["max"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

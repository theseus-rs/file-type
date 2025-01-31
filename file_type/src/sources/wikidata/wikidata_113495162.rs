use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113495162: FileFormat = FileFormat {
    id: 113_495_162,
    puid: "wikidata/113495162",
    name: "Calc602 Project File 1.0",
    extensions: &["pc6"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

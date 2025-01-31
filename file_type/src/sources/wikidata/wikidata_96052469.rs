use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_96052469: FileFormat = FileFormat {
    id: 96_052_469,
    puid: "wikidata/96052469",
    name: "Mathematica Graphics Format",
    extensions: &["mgf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

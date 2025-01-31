use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130130523: FileFormat = FileFormat {
    id: 130_130_523,
    puid: "wikidata/130130523",
    name: "Kuin source code file",
    extensions: &["kn"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

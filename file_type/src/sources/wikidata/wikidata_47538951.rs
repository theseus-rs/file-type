use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47538951: FileFormat = FileFormat {
    id: 47_538_951,
    puid: "wikidata/47538951",
    name: "AutoCAD Compiled Menu",
    extensions: &["met"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

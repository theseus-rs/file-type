use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_73794214: FileFormat = FileFormat {
    id: 73_794_214,
    puid: "wikidata/73794214",
    name: "QGIS Layer",
    extensions: &["qlr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

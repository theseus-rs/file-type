use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_73794214: FileFormat = FileFormat {
    id: 73_794_214,
    source_type: SourceType::Wikidata,
    name: "QGIS Layer",
    extensions: &["qlr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

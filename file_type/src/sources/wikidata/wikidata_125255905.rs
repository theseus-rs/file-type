use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125255905: FileFormat = FileFormat {
    id: 125_255_905,
    puid: "wikidata/125255905",
    name: "Simulation Result File",
    extensions: &["mat"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

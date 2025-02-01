use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_122412029: FileFormat = FileFormat {
    id: 122_412_029,
    puid: "wikidata/122412029",
    name: "FileMaker Runtime File",
    extensions: &["syo"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

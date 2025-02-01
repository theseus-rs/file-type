use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_49620191: FileFormat = FileFormat {
    id: 49_620_191,
    puid: "wikidata/49620191",
    name: "Revit Project",
    extensions: &["rvt"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};

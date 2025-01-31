use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_49620373: FileFormat = FileFormat {
    id: 49_620_373,
    puid: "wikidata/49620373",
    name: "Revit Workspace",
    extensions: &["rws"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};

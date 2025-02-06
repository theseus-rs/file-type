use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_49620373: FileFormat = FileFormat {
    id: 49_620_373,
    source_type: SourceType::Wikidata,
    name: "Revit Workspace",
    extensions: &["rws"],
    media_types: &["application/octet-stream"],
    signatures: &[],
    related_formats: &[],
};

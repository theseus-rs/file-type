use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_49620191: FileFormat = FileFormat {
    id: 49_620_191,
    source_type: SourceType::Wikidata,
    name: "Revit Project",
    extensions: &["rvt"],
    media_types: &["application/octet-stream"],
    signatures: &[],
    related_formats: &[],
};

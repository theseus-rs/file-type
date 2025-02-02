use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_49620191: FileFormat = FileFormat {
    id: 49_620_191,
    source_type: SourceType::Wikidata,
    name: "Revit Project",
    extensions: &["rvt"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};

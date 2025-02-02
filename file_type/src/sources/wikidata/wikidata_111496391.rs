use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111496391: FileFormat = FileFormat {
    id: 111_496_391,
    source_type: SourceType::Wikidata,
    name: "Visual Basic Project Workspace File",
    extensions: &["vbw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

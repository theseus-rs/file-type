use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111496391: FileFormat = FileFormat {
    id: 111_496_391,
    source_type: SourceType::Wikidata,
    name: "Visual Basic Project Workspace File",
    extensions: &["vbw"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

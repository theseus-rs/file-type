use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_47539012: FileFormat = FileFormat {
    id: 47_539_012,
    source_type: SourceType::Wikidata,
    name: "AutoCAD Drawing Template",
    extensions: &["dwt"],
    media_types: &["application/x-autocad"],
    internal_signatures: &[],
    related_formats: &[],
};

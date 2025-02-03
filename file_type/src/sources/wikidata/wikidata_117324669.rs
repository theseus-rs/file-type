use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117324669: FileFormat = FileFormat {
    id: 117_324_669,
    source_type: SourceType::Wikidata,
    name: "LabWindows/CVI Project file",
    extensions: &["prj"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

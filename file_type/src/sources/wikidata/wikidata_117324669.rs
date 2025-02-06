use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117324669: FileFormat = FileFormat {
    id: 117_324_669,
    source_type: SourceType::Wikidata,
    name: "LabWindows/CVI Project file",
    extensions: &["prj"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

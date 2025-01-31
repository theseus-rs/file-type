use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_445: FileFormat = FileFormat {
    id: 1_232,
    puid: "fmt/445",
    name: "Microsoft Excel Macro-Enabled",
    extensions: &["xlsm"],
    media_types: &["application/vnd.ms-excel.sheet.macroEnabled.12"],
    internal_signatures: &[],
    related_formats: &[],
};

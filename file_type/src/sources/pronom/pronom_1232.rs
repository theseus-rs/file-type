use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1232: FileFormat = FileFormat {
    id: 1_232,
    source_type: SourceType::Pronom,
    name: "Microsoft Excel Macro-Enabled",
    extensions: &["xlsm"],
    media_types: &["application/vnd.ms-excel.sheet.macroEnabled.12"],
    signatures: &[],
    related_formats: &[],
};

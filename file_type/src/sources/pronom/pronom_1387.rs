use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1387: FileFormat = FileFormat {
    id: 1_387,
    source_type: SourceType::Pronom,
    name: "Microsoft Excel Non-XML Binary Workbook",
    extensions: &["xlsb"],
    media_types: &["application/vnd.ms-excel.sheet.binary.macroEnabled.12"],
    signatures: &[],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::IsSubtypeOf,
        id: 940,
    }],
};

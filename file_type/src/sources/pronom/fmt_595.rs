use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_595: FileFormat = FileFormat {
    id: 1_387,
    puid: "fmt/595",
    name: "Microsoft Excel Non-XML Binary Workbook",
    extensions: &["xlsb"],
    media_types: &["application/vnd.ms-excel.sheet.binary.macroEnabled.12"],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        id: 940,
        relationship_type: RelationshipType::IsSubtypeOf,
    }],
};

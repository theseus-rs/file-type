use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2112: FileFormat = FileFormat {
    id: 2_112,
    source_type: SourceType::Pronom,
    name: "602Tab Spreadsheet",
    extensions: &["wls"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

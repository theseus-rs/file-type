use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_265: FileFormat = FileFormat {
    id: 265,
    source_type: SourceType::Pronom,
    name: "SAS for MS-DOS Catalog",
    extensions: &["sct"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

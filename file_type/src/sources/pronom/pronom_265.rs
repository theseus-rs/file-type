use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_265: FileFormat = FileFormat {
    id: 265,
    source_type: SourceType::Pronom,
    name: "SAS for MS-DOS Catalog",
    extensions: &["sct"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

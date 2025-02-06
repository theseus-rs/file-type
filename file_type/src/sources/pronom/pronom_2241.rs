use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2241: FileFormat = FileFormat {
    id: 2_241,
    source_type: SourceType::Pronom,
    name: "HP TRIM Outlook Saved Message File",
    extensions: &["vmbx", "mbx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

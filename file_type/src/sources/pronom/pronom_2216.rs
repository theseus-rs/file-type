use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2216: FileFormat = FileFormat {
    id: 2_216,
    source_type: SourceType::Pronom,
    name: "FARO WorkSpace File",
    extensions: &["fws"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

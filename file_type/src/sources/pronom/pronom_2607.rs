use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2607: FileFormat = FileFormat {
    id: 2_607,
    source_type: SourceType::Pronom,
    name: "Media Descriptor File",
    extensions: &["mdf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

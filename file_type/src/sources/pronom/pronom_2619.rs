use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2619: FileFormat = FileFormat {
    id: 2_619,
    source_type: SourceType::Pronom,
    name: "C++ Source Code File",
    extensions: &["cpp", "cxx", "cc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

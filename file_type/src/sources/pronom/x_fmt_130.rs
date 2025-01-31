use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_130: FileFormat = FileFormat {
    id: 189,
    puid: "x-fmt/130",
    name: "MS-DOS Text File with line breaks",
    extensions: &[],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};

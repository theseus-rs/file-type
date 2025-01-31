use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_135: FileFormat = FileFormat {
    id: 194,
    puid: "x-fmt/135",
    name: "Audio Interchange File Format",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

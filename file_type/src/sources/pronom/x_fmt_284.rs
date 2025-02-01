use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_284: FileFormat = FileFormat {
    id: 435,
    puid: "x-fmt/284",
    name: "IBM DisplayWrite Final Form Text File",
    extensions: &["fft"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

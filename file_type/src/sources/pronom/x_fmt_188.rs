use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_188: FileFormat = FileFormat {
    id: 261,
    puid: "x-fmt/188",
    name: "SDSC Image Tool Wavefront Raster Image",
    extensions: &["rla"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

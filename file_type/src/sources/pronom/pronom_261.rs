use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_261: FileFormat = FileFormat {
    id: 261,
    source_type: SourceType::Pronom,
    name: "SDSC Image Tool Wavefront Raster Image",
    extensions: &["rla"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_261: FileFormat = FileFormat {
    id: 261,
    source_type: SourceType::Pronom,
    name: "SDSC Image Tool Wavefront Raster Image",
    extensions: &["rla"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

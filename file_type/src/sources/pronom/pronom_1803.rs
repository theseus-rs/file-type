use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1803: FileFormat = FileFormat {
    id: 1_803,
    source_type: SourceType::Pronom,
    name: "OpenRaster Image Format",
    extensions: &["ora"],
    media_types: &["image/openraster"],
    internal_signatures: &[],
    related_formats: &[],
};

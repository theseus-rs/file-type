use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_920: FileFormat = FileFormat {
    id: 920,
    source_type: SourceType::Pronom,
    name: "ERDAS IMAGINE Gray-scale Bitmap Image",
    extensions: &["gis"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

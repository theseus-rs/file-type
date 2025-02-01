use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_195: FileFormat = FileFormat {
    id: 920,
    puid: "fmt/195",
    name: "ERDAS IMAGINE Gray-scale Bitmap Image",
    extensions: &["gis"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

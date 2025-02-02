use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_140: FileFormat = FileFormat {
    id: 140,
    source_type: SourceType::Pronom,
    name: "Postscript Support File",
    extensions: &["psf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

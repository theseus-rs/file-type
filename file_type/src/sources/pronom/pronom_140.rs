use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_140: FileFormat = FileFormat {
    id: 140,
    source_type: SourceType::Pronom,
    name: "Postscript Support File",
    extensions: &["psf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

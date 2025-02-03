use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_814: FileFormat = FileFormat {
    id: 814,
    source_type: SourceType::Pronom,
    name: "Acrobat Language definition file",
    extensions: &["lng"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

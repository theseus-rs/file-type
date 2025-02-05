use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_814: FileFormat = FileFormat {
    id: 814,
    source_type: SourceType::Pronom,
    name: "Acrobat Language definition file",
    extensions: &["lng"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

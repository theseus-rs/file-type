use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1804: FileFormat = FileFormat {
    id: 1_804,
    source_type: SourceType::Pronom,
    name: "Krita Document Format",
    extensions: &["kra"],
    media_types: &["application/x-krita"],
    signatures: &[],
    related_formats: &[],
};

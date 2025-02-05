use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_967: FileFormat = FileFormat {
    id: 967,
    source_type: SourceType::Pronom,
    name: "Microsoft Office Binder File for Windows",
    extensions: &["obd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

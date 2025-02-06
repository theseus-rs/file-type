use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_970: FileFormat = FileFormat {
    id: 970,
    source_type: SourceType::Pronom,
    name: "Microsoft Office Binder File for Windows",
    extensions: &["obd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

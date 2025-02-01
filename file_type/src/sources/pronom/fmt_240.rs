use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_240: FileFormat = FileFormat {
    id: 970,
    puid: "fmt/240",
    name: "Microsoft Office Binder File for Windows",
    extensions: &["obd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

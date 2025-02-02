use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_131: FileFormat = FileFormat {
    id: 131,
    source_type: SourceType::Pronom,
    name: "Microsoft Powerpoint Add-In",
    extensions: &["ppa"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

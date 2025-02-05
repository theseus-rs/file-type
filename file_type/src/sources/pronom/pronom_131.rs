use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_131: FileFormat = FileFormat {
    id: 131,
    source_type: SourceType::Pronom,
    name: "Microsoft Powerpoint Add-In",
    extensions: &["ppa"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

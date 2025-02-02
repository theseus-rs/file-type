use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2180: FileFormat = FileFormat {
    id: 2_180,
    source_type: SourceType::Pronom,
    name: "Microsoft MapPoint Document",
    extensions: &["ptm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

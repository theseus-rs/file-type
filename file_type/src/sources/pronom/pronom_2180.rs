use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2180: FileFormat = FileFormat {
    id: 2_180,
    source_type: SourceType::Pronom,
    name: "Microsoft MapPoint Document",
    extensions: &["ptm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

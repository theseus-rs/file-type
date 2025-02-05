use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_132: FileFormat = FileFormat {
    id: 132,
    source_type: SourceType::Pronom,
    name: "Microsoft Powerpoint Presentation Show",
    extensions: &["pps"],
    media_types: &["application/vnd.ms-powerpoint"],
    signatures: &[],
    related_formats: &[],
};

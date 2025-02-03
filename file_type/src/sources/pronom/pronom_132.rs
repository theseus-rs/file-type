use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_132: FileFormat = FileFormat {
    id: 132,
    source_type: SourceType::Pronom,
    name: "Microsoft Powerpoint Presentation Show",
    extensions: &["pps"],
    media_types: &["application/vnd.ms-powerpoint"],
    internal_signatures: &[],
    related_formats: &[],
};

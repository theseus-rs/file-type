use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_366: FileFormat = FileFormat {
    id: 366,
    source_type: SourceType::Pronom,
    name: "Microsoft Outlook Personal Folders",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_367: FileFormat = FileFormat {
    id: 367,
    source_type: SourceType::Pronom,
    name: "Microsoft Outlook Personal Folders",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

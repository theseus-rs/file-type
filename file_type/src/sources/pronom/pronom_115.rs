use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_115: FileFormat = FileFormat {
    id: 115,
    source_type: SourceType::Pronom,
    name: "Microsoft Outlook Address Book",
    extensions: &["olk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

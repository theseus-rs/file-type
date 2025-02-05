use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_115: FileFormat = FileFormat {
    id: 115,
    source_type: SourceType::Pronom,
    name: "Microsoft Outlook Address Book",
    extensions: &["olk"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

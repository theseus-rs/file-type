use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_367: FileFormat = FileFormat {
    id: 367,
    source_type: SourceType::Pronom,
    name: "Microsoft Outlook Personal Folders",
    extensions: &[],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_366: FileFormat = FileFormat {
    id: 366,
    source_type: SourceType::Pronom,
    name: "Microsoft Outlook Personal Folders",
    extensions: &[],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

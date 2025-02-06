use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_375: FileFormat = FileFormat {
    id: 375,
    source_type: SourceType::Pronom,
    name: "Microsoft Publisher",
    extensions: &["pub"],
    media_types: &["application/x-mspublisher"],
    signatures: &[],
    related_formats: &[],
};

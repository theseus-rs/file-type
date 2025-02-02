use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_371: FileFormat = FileFormat {
    id: 371,
    source_type: SourceType::Pronom,
    name: "Microsoft Publisher",
    extensions: &["pub"],
    media_types: &["application/x-mspublisher"],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_372: FileFormat = FileFormat {
    id: 372,
    source_type: SourceType::Pronom,
    name: "Microsoft Publisher",
    extensions: &["pub"],
    media_types: &["application/x-mspublisher"],
    signatures: &[],
    related_formats: &[],
};

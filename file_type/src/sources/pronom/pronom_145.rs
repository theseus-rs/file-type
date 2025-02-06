use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_145: FileFormat = FileFormat {
    id: 145,
    source_type: SourceType::Pronom,
    name: "Microsoft Excel OLE DB Query",
    extensions: &["rqy"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

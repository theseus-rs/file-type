use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_145: FileFormat = FileFormat {
    id: 145,
    source_type: SourceType::Pronom,
    name: "Microsoft Excel OLE DB Query",
    extensions: &["rqy"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

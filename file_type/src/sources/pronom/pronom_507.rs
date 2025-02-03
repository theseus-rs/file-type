use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_507: FileFormat = FileFormat {
    id: 507,
    source_type: SourceType::Pronom,
    name: "Microsoft Visual FoxPro Table",
    extensions: &["dbx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

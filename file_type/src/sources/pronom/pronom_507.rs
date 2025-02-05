use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_507: FileFormat = FileFormat {
    id: 507,
    source_type: SourceType::Pronom,
    name: "Microsoft Visual FoxPro Table",
    extensions: &["dbx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

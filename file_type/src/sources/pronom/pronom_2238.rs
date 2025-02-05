use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2238: FileFormat = FileFormat {
    id: 2_238,
    source_type: SourceType::Pronom,
    name: "Corel Print House/Print Office Document",
    extensions: &["cph", "cpd", "cpo"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

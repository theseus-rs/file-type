use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2325: FileFormat = FileFormat {
    id: 2_325,
    source_type: SourceType::Pronom,
    name: "Agisoft Project Archive",
    extensions: &["psz"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

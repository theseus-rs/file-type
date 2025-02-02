use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_754: FileFormat = FileFormat {
    id: 754,
    source_type: SourceType::Pronom,
    name: "StarOffice Writer",
    extensions: &["sdw"],
    media_types: &["application/vnd.stardivision.writer"],
    internal_signatures: &[],
    related_formats: &[],
};

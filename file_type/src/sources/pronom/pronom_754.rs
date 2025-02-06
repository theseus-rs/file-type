use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_754: FileFormat = FileFormat {
    id: 754,
    source_type: SourceType::Pronom,
    name: "StarOffice Writer",
    extensions: &["sdw"],
    media_types: &["application/vnd.stardivision.writer"],
    signatures: &[],
    related_formats: &[],
};

use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_755: FileFormat = FileFormat {
    id: 755,
    source_type: SourceType::Pronom,
    name: "StarOffice Draw",
    extensions: &["sda"],
    media_types: &["application/vnd.stardivision.draw"],
    internal_signatures: &[],
    related_formats: &[],
};

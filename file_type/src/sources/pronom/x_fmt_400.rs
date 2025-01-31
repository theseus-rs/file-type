use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_400: FileFormat = FileFormat {
    id: 754,
    puid: "x-fmt/400",
    name: "StarOffice Writer",
    extensions: &["sdw"],
    media_types: &["application/vnd.stardivision.writer"],
    internal_signatures: &[],
    related_formats: &[],
};

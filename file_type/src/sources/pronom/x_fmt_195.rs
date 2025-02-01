use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_195: FileFormat = FileFormat {
    id: 268,
    puid: "x-fmt/195",
    name: "Standard Generalized Markup Language",
    extensions: &["sgml", "sgm"],
    media_types: &["text/sgml"],
    internal_signatures: &[],
    related_formats: &[],
};

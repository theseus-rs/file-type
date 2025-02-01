use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_326: FileFormat = FileFormat {
    id: 489,
    puid: "x-fmt/326",
    name: "Hewlett Packard AdvanceWrite Text File",
    extensions: &["aw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

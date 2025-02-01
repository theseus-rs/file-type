use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_177: FileFormat = FileFormat {
    id: 250,
    puid: "x-fmt/177",
    name: "Microsoft PowerPoint Graphics File",
    extensions: &["ppi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

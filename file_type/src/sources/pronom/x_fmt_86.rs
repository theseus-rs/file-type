use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_86: FileFormat = FileFormat {
    id: 131,
    puid: "x-fmt/86",
    name: "Microsoft Powerpoint Add-In",
    extensions: &["ppa"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

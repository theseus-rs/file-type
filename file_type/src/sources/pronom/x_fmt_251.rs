use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_251: FileFormat = FileFormat {
    id: 367,
    puid: "x-fmt/251",
    name: "Microsoft Outlook Personal Folders",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

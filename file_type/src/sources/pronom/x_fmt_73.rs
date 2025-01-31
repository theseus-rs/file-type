use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_73: FileFormat = FileFormat {
    id: 115,
    puid: "x-fmt/73",
    name: "Microsoft Outlook Address Book",
    extensions: &["olk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

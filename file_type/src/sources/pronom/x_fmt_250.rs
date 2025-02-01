use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_250: FileFormat = FileFormat {
    id: 366,
    puid: "x-fmt/250",
    name: "Microsoft Outlook Personal Folders",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

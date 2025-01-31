use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_87: FileFormat = FileFormat {
    id: 132,
    puid: "x-fmt/87",
    name: "Microsoft Powerpoint Presentation Show",
    extensions: &["pps"],
    media_types: &["application/vnd.ms-powerpoint"],
    internal_signatures: &[],
    related_formats: &[],
};

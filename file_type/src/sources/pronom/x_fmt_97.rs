use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_97: FileFormat = FileFormat {
    id: 145,
    puid: "x-fmt/97",
    name: "Microsoft Excel OLE DB Query",
    extensions: &["rqy"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

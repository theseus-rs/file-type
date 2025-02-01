use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_343: FileFormat = FileFormat {
    id: 507,
    puid: "x-fmt/343",
    name: "Microsoft Visual FoxPro Table",
    extensions: &["dbx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

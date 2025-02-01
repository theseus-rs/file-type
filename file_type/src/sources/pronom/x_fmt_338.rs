use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_338: FileFormat = FileFormat {
    id: 501,
    puid: "x-fmt/338",
    name: "Lotus Notes Database",
    extensions: &["ns4", "nsf"],
    media_types: &["application/vnd.lotus-notes"],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_336: FileFormat = FileFormat {
    id: 499,
    puid: "x-fmt/336",
    name: "Lotus Notes Database",
    extensions: &["ns2", "nsf"],
    media_types: &["application/vnd.lotus-notes"],
    internal_signatures: &[],
    related_formats: &[],
};

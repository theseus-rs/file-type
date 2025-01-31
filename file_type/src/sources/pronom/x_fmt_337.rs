use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_337: FileFormat = FileFormat {
    id: 500,
    puid: "x-fmt/337",
    name: "Lotus Notes Database",
    extensions: &["ns3", "nsf"],
    media_types: &["application/vnd.lotus-notes"],
    internal_signatures: &[],
    related_formats: &[],
};

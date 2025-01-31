use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_916: FileFormat = FileFormat {
    id: 1_721,
    puid: "fmt/916",
    name: "ESRI ArcMap Document",
    extensions: &["mxd", "mxt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

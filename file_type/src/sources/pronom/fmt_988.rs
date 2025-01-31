use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_988: FileFormat = FileFormat {
    id: 1_793,
    puid: "fmt/988",
    name: "ESRI ArcScene Document",
    extensions: &["sxd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

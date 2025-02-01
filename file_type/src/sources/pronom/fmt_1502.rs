use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1502: FileFormat = FileFormat {
    id: 2_325,
    puid: "fmt/1502",
    name: "Agisoft Project Archive",
    extensions: &["psz"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
